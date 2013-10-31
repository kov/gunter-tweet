#!/usr/bin/env python

import os
import random
import tweepy

import config


last_seen_path = os.path.join(os.path.dirname(__file__), 'last-seen')


def get_api():
    auth = tweepy.OAuthHandler(config.consumer_key, config.consumer_secret)
    auth.set_access_token(config.key, config.secret)

    return tweepy.API(auth)


def get_last_seen():
    try:
        return int(open(last_seen_path).read())
    except:
        pass


def save_last_seen(mentions):
    open(last_seen_path, 'w').write(str(mentions[-1].id))


def generate_wenks():
    return ' '.join(['Wenk'] * random.randrange(1, 4))


def generate_reply(mention):
    return '@' + mention.user.screen_name + ' ' + generate_wenks()


def should_wenk():
    # 10% probability
    return random.randrange(-9, 1) == 0


api = get_api()

mentions = api.mentions_timeline(since_id=get_last_seen())
if mentions:
    for mention in reversed(mentions):
        api.update_status(generate_reply(mention))

    save_last_seen(mentions)
elif should_wenk():
    api.update_status(generate_wenks())

