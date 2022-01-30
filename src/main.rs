// Copyright © 2022 Gustavo Noronha <gustavo@noronha.dev.br>
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU Affero General Public License as
//  published by the Free Software Foundation, either version 3 of the
//  License, or (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU Affero General Public License for more details.
//
//  You should have received a copy of the GNU Affero General Public License
//  along with this program.  If not, see <http://www.gnu.org/licenses/>.
use lambda_runtime::{handler_fn, Context, Error};
use rand::{Rng, distributions::Distribution};
use regex::Regex;
use serde::{Deserialize, Serialize};
use storage::{AwsSsmStorage, FileStorage, GunterStorage};

mod config;
mod storage;

const POSSIBLE_SENTENCES: &[(&str, u32)] = &[
    ("Wenk.", 6),
    ("Wenk, wenk.", 3),
    ("Wenk, wenk, wenk.", 1)
];

struct Gunter {
    token: egg_mode::Token,
    storage: Box<dyn GunterStorage>,
    gunter_regex: regex::Regex,
    advtime_regex: regex::Regex,
}

impl Gunter {
    fn new(storage: Box<dyn GunterStorage>) -> Self {
        let consumer_token = egg_mode::KeyPair::new(config::CONSUMER_KEY, config::CONSUMER_SECRET);
        let access_token = egg_mode::KeyPair::new(config::KEY, config::SECRET);
        let token = egg_mode::Token::Access {
            consumer: consumer_token,
            access: access_token,
        };

        let gunter_regex = Regex::new(
            r"\b(gunter|#gunter)\b"
        ).expect("Failed to compile regex");

        let advtime_regex = Regex::new(
            r"\b(ice king|adventure time|adventuretime|#adventuretime|#adventuretimeweek)\b"
        ).expect("Failed to compile regex");

        Gunter {
            token,
            storage,
            gunter_regex,
            advtime_regex,
        }
    }

    fn should_reply(&self, tweet: &egg_mode::tweet::Tweet) -> bool {
        let text = tweet.text.to_lowercase();

        if !self.gunter_regex.is_match(&text) {
            return false;
        }

        if !self.advtime_regex.is_match(&text) {
            return false;
        }

        true
    }

    fn should_wenk(&self) -> bool {
        rand::thread_rng().gen_range(0..18) == 0
    }

    fn generate_wenks(&self) -> String {
        let mut rng = rand::thread_rng();

        let number_of_sentences: u32 = rng.gen_range(1..4);
        let mut sentences = vec![];
        for _ in 0..number_of_sentences {
            let distribution = rand::distributions::WeightedIndex::new(
                POSSIBLE_SENTENCES.iter().map(|item| item.1)
            ).unwrap();

            sentences.push(
                POSSIBLE_SENTENCES[distribution.sample(&mut rng)].0.to_string()
            );
        }

        sentences.join(" ")
    }

    async fn wenk(&self) {
        egg_mode::tweet::DraftTweet::new(self.generate_wenks())
            .send(&self.token).await
            .unwrap_or_else(|e| panic!("Failed to wenk {}", e));
    }

    async fn reply_to(&self, tweet: &egg_mode::tweet::Tweet) {
        egg_mode::tweet::DraftTweet::new(self.generate_wenks())
            .in_reply_to(tweet.id)
            .auto_populate_reply_metadata(true)
            .send(&self.token).await
            .unwrap_or_else(|e| panic!("Failed to send tweet: {}", e));
    }

    async fn run(&self) {
        // Begin by replying to any mentions.
        let mentions = egg_mode::tweet::mentions_timeline(&self.token);

        let mentions = mentions.call(
            Some(self.storage.last_seen().await),
            None,
        ).await.expect("Failed to obtain mentions timeline");

        if !mentions.is_empty() {
            self.storage.save_last_seen(mentions[0].id).await;

            for tweet in mentions.iter().rev() {
                if let Some(user) = &tweet.user {
                    // We do not want to, you know, enter an infinite reply loop.
                    if user.screen_name == "GunterWenkWenk" {
                        continue;
                    }

                    self.reply_to(tweet).await;
                }
            }

            return;
        }

        // Then, see if we can find any interesting tweets to reply to.
        let search = egg_mode::search::search("gunter")
            .result_type(egg_mode::search::ResultType::Recent)
            .since_tweet(self.storage.last_search().await)
            .call(&self.token)
            .await
            .unwrap();

        if !search.statuses.is_empty() {
            self.storage.save_last_search(search.statuses[0].id).await;

            let mut number_of_replies = 0;
            for tweet in search.statuses.iter().rev() {
                if !self.should_reply(tweet) {
                    continue;
                }

                self.reply_to(tweet).await;

                number_of_replies += 1;

                // Keep the number of replies civil.
                if number_of_replies > 2 {
                    break;
                }
            }
        }

        // We don't want to wenk every run, it gets too noisy.
        if !self.should_wenk() {
            return;
        }

        self.wenk().await;
    }
}

#[derive(Deserialize, Serialize)]
struct LambdaMsg {}

async fn handler(_request: LambdaMsg, _context: Context) -> Result<LambdaMsg, Error> {
    Gunter::new(
        Box::new(AwsSsmStorage::new().await)
    ).run().await;
    Ok(LambdaMsg {})
}

fn is_standalone() -> bool {
    let executable = std::env::current_exe()
        .expect("Could not find my executable's path")
        .file_name()
        .expect("Could not find my executable's filename")
        .to_string_lossy()
        .to_string();

    executable != "bootstrap"
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    if is_standalone() {
        Gunter::new(
            Box::new(FileStorage::new())
        ).run().await;
    } else {
        lambda_runtime::run(handler_fn(handler)).await?;
    }
    Ok(())
}