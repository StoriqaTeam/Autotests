#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import requests
import json
import os
import graph_queries as q
from datetime import datetime


if os.getenv('GRAPHQL_URL'):
    url = os.environ['GRAPHQL_URL']
else: url = 'https://nightly.stq.cloud/graphql'


def request(json_query, headers, cookies):
    r = requests.post(url, json=json_query, headers=headers, cookies=cookies)
    return r

def action(dictq):
    token_headers = ''
    errors = {}
    cookie = {"holyshit": "iamcool"}
    answer: json
    count = 0
    context = {
        'n': datetime.strftime(datetime.now(), "%m%d%H%M%S"),
    }
    context['regmail'] = 'test' + context['n'] + '@test.test'
    for i in dictq:
        answer = request(json.loads(dictq[i] % context), token_headers, cookie)
        if dictq[i] == q.queries['adm_token']:
            ad_token = answer.json()['data']['getJWTByEmail']['token']
            token_headers = {'Authorization': 'Bearer ' + ad_token}
        elif dictq[i] == q.queries['cr_cat']:
            context['cat_id'] = answer.json()['data']['createCategory']['id']
            context['cat_rawid'] = answer.json()['data']['createCategory']['rawId']
        elif dictq[i] == q.queries['cr_attr']:
            context['attr_id'] = answer.json()['data']['createAttribute']['id']
            context['attr_rawid'] = answer.json()['data']['createAttribute']['rawId']
        elif dictq[i] == q.queries['user_token']:
            token = answer.json()['data']['getJWTByEmail']['token']
            token_headers = {'Authorization': 'Bearer '+ token}
        elif dictq[i] == q.queries['user_id']:
            context['usr_id'] = answer.json()['data']['me']['id']
            context['usr_rawId'] = answer.json()['data']['me']['rawId']
        elif dictq[i] == q.queries['createUserDeliveryAddress']:
            context['addr_id'] = answer.json()['data']['createUserDeliveryAddress']['id']
            context['addr_rawid'] = answer.json()['data']['createUserDeliveryAddress']['rawId']
        elif dictq[i] == q.queries['cr_store']:
            context['store_id'] = answer.json()['data']['createStore']['id']
            context['store_rawid'] = answer.json()['data']['createStore']['rawId']
        elif dictq[i] == q.queries['cr_warehouse']:
            context['war_id'] = answer.json()['data']['createWarehouse']['id']
        elif dictq[i] == q.queries['cr_b_prod']:
            context['b_prod_id'] = answer.json()['data']['createBaseProduct']['id']
            context['b_prod_rawid'] = answer.json()['data']['createBaseProduct']['rawId']
        elif dictq[i] == q.queries['cr_prod']:
            context['prod_id'] = answer.json()['data']['createProduct']['id']
            context['prod_rawid'] = answer.json()['data']['createProduct']['rawId']
        elif dictq[i] == q.queries['cr_order']:
            context['order_slug'] = answer.json()['data']['createOrders']['invoice']['orders'][0]['slug']
        print(answer.json())
        if 'errors' in answer.text:
            raise Exception(answer.text)


action(q.queries)
