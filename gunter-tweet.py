#!/usr/bin/env python

import os
import random
import tweepy

import config


last_seen_path = os.path.join(os.path.dirname(__file__), 'last-seen')
last_seen_search_path = os.path.join(os.path.dirname(__file__), 'last-seen-search')


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
    open(last_seen_path, 'w').write(str(mentions[0].id))


def get_last_seen_search():
    try:
        return int(open(last_seen_search_path).read())
    except:
        pass


def save_last_seen_search(results):
    open(last_seen_search_path, 'w').write(str(results[0].id))


def generate_wenks():
    return ' '.join(['Wenk.'] * random.randrange(1, 4))


def generate_reply(mention):
    return '@' + mention.user.screen_name + ' ' + generate_wenks()


def should_wenk():
    # 10% probability
    return random.randrange(-9, 1) == 0


api = get_api()

mentions = api.mentions_timeline(since_id=get_last_seen())

if mentions:
    for mention in reversed(mentions):
        try:
            api.update_status(generate_reply(mention), in_reply_to_status_id=mention.id)
        except tweepy.error.TweepError, e:
            print e.message
    save_last_seen(mentions)
elif should_wenk():
    api.update_status(generate_wenks())

search_results = api.search('gunter', since_id=get_last_seen_search())
if search_results:
    for result in search_results:
        if 'gunter' in result.text.lower() and result.user.screen_name != 'GunterWenkWenk':
            try:
                api.update_status(generate_reply(result), in_reply_to_status_id=result.id)
            except tweepy.error.TweepError, e:
                print e.message
    save_last_seen_search(search_results)

