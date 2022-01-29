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
use async_trait::async_trait;
use super::GunterStorage;

pub struct AwsSsmStorage {
    client: aws_sdk_ssm::Client,
}

impl AwsSsmStorage {
    pub async fn new() -> Self {
        let shared_config = aws_config::load_from_env().await;
        let client = aws_sdk_ssm::Client::new(&shared_config);

        AwsSsmStorage {
            client
        }
    }

    async fn get_parameter(&self, name: &str) -> u64 {
        self.client.get_parameter()
            .set_name(Some(name.to_string()))
            .send()
            .await
            .expect("Failed to query parameter")
            .parameter
            .expect("Could not find parameter")
            .value
            .expect("Parameter value is None")
            .parse::<u64>()
            .expect("Parameter value is not a valid u64")
    }

    async fn set_parameter(&self, name: &str, value: u64) {
        self.client.put_parameter()
            .set_name(Some(name.to_string()))
            .set_value(Some(format!("{}", value)))
            .set_overwrite(Some(true))
            .send()
            .await
            .expect("Failed to put parameter");
    }
}

#[async_trait]
impl GunterStorage for AwsSsmStorage {
    async fn last_seen(&self) -> u64 {
        self.get_parameter("last-seen").await
    }

    async fn save_last_seen(&self, timestamp: u64) {
        self.set_parameter("last-seen", timestamp).await
    }

    async fn last_search(&self) -> u64 {
        self.get_parameter("last-seen-search").await
    }

    async fn save_last_search(&self, timestamp: u64) {
        self.set_parameter("last-seen-search", timestamp).await
    }
}