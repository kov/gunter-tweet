#!/usr/bin/env python
# -*- coding: utf-8 -*-
#
# Copyright (©) 2013 Gustavo Noronha <gustavo@noronha.eti.br>
#
#  This program is free software: you can redistribute it and/or modify
#  it under the terms of the GNU Affero General Public License as
#  published by the Free Software Foundation, either version 3 of the
#  License, or (at your option) any later version.
#
#  This program is distributed in the hope that it will be useful,
#  but WITHOUT ANY WARRANTY; without even the implied warranty of
#  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#  GNU Affero General Public License for more details.
#
#  You should have received a copy of the GNU Affero General Public License
#  along with this program.  If not, see <http://www.gnu.org/licenses/>.

import httplib
import os
import random
import re
import tweepy

import config


Nothing = None
SendReply = 1
Retweet = 2


last_seen_path = os.path.join(os.path.dirname(__file__), 'last-seen')
last_seen_search_path = os.path.join(os.path.dirname(__file__), 'last-seen-search')
replies_this_round = 0


def too_many_replies():
    return replies_this_round > 2


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


def generate_reply(status):
    reply = '@' + status.user.screen_name + ' ' + generate_wenks()
    for update in user_timeline:
        if update.text == reply:
            reply = ' Wenk.'
    return reply


def should_wenk():
    return random.randrange(-18, 1) == 0


def has_gunter(status):
    text = status.text.lower()
    if not 'gunter' in text:
        return False

    if not bool(re.search(r'\bgunter\b', text)) and not bool(re.search(r'\b#gunter\b', text)):
        return False

    words = ['ice king', 'adventure time', 'adventuretime', '#adventuretime',
             '#adventuretimeweek', 'hora de aventuras', 'horadeaventuras', '#horadeaventuras']

    for word in words:
        if re.search(r'\b%s\b' % word, text):
            return True

    return False


def resolve_url(url):
    connection = httplib.HTTPConnection('t.co')
    connection.request('GET', url)
    headers = connection.getresponse().getheaders()
    for t in headers:
        if t[0] == 'location':
            return t[1]
    return url


def resolve_urls(text):
    urls = re.findall('http://t\.co/[0-9A-Za-z]{10}', text)
    if not urls:
        return text

    for url in urls:
        resolved = resolve_url(url)
        if url != resolved:
            text = text.replace(url, resolved)

    return text


def what_to_do_with(status):
    if not has_gunter(status):
        return Nothing

    text = resolve_urls(status.text)

    for picsite in ['instagram.com', 'pic.twitter.com']:
        if picsite in text:
            return Retweet

    return SendReply


def already_replied_to(status):
    for update in user_timeline:
        if update.in_reply_to_status_id == status.id:
            return True
    return False


def send_reply(status):
    if already_replied_to(status):
        return

    global replies_this_round
    replies_this_round += 1

    reply = generate_reply(status)
    try:
        api.update_status(reply, in_reply_to_status_id=status.id)
        print 'Replied', reply, 'to', status.text, 'by', status.user.screen_name
    except tweepy.error.TweepError, e:
        print status.text, '=>', reply, e.message


api = get_api()
user_timeline = api.user_timeline()

mentions = api.mentions_timeline(since_id=get_last_seen())

if mentions:
    for mention in reversed(mentions):
        send_reply(mention)

    save_last_seen(mentions)
elif should_wenk():
    api.update_status(generate_wenks())

search_results = api.search('gunter', since_id=get_last_seen_search())
if search_results:
    for result in search_results:
        what = what_to_do_with(result)
        if what == SendReply and not too_many_replies():
            send_reply(result)
        elif what == Retweet:
            result.retweet()
            print 'Retweeted', result.text, 'by', result.user.screen_name

    save_last_seen_search(search_results)

