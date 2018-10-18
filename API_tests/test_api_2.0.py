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

class TestFailException(Exception):
    pass

errors = {}

def request(json_query, headers, cookies):
    r = requests.post(url, json=json_query, headers=headers, cookies=cookies)
    return r

def action(dictq):
    token_headers = {"currency" : "STQ"}
    cookie = {"holyshit": "iamcool"}
    answer: json
    count = 0
    context = {
        'n': datetime.strftime(datetime.now(), "%m%d%H%M%S"),
        'store_id': ""
    }
    context['regmail'] = 'test' + context['n'] + '@test.test'
    for i in dictq:
        try:
            answer = request(json.loads(dictq[i] % context), token_headers, cookie)
            if dictq[i] == q.queries['adm_token']:
                ad_token = 'Bearer ' + answer.json()['data']['getJWTByEmail']['token']
                token_headers['Authorization'] =  ad_token
            elif dictq[i] == q.queries['cr_cat1']:
                context['cat_id_1'] = answer.json()['data']['createCategory']['id']
                context['cat_rawid_1'] = answer.json()['data']['createCategory']['rawId']
            elif dictq[i] == q.queries['cr_cat2']:
                context['cat_id_2'] = answer.json()['data']['createCategory']['id']
                context['cat_rawid_2'] = answer.json()['data']['createCategory']['rawId']
            elif dictq[i] == q.queries['cr_cat3']:
                context['cat_id_3'] = answer.json()['data']['createCategory']['id']
                context['cat_rawid_3'] = answer.json()['data']['createCategory']['rawId']
            elif dictq[i] == q.queries['cr_attr']:
                context['attr_id'] = answer.json()['data']['createAttribute']['id']
                context['attr_rawid'] = answer.json()['data']['createAttribute']['rawId']
            elif dictq[i] == q.queries['cr_company']:
                context['company_id'] = answer.json()['data']['createCompany']['id']
                context['company_rawid'] = answer.json()['data']['createCompany']['rawId']
            elif dictq[i] == q.queries['cr_package']:
                context['package_id'] = answer.json()['data']['createPackage']['id']
                context['package_rawid'] = answer.json()['data']['createPackage']['rawId']
            elif dictq[i] == q.queries['ad_package']:
                context['comp_pack_rawid'] = answer.json()['data']['addPackageToCompany']['rawId']
            elif dictq[i] == q.queries['cr_user']:
                context['n_usr_rawid'] = answer.json()['data']['createUser']['rawId']
            elif dictq[i] == q.queries['user_token']:
                token = 'Bearer ' +  answer.json()['data']['getJWTByEmail']['token']
                token_headers['Authorization'] = token
            elif dictq[i] == q.queries['user_id']:
                context['usr_id'] = answer.json()['data']['me']['id']
                context['usr_rawid'] = answer.json()['data']['me']['rawId']
                try:
                    context['store_id'] = answer.json()['data']['me']['myStore']['id']
                    if len(context['store_id']) > 0:
                        request(json.loads(dictq['deact_store'] % context), token_headers, cookie)
                except: TypeError()
            elif dictq[i] == q.queries['createUserDeliveryAddressFull']:
                context['addr_id'] = answer.json()['data']['createUserDeliveryAddressFull']['id']
                context['addr_rawid'] = answer.json()['data']['createUserDeliveryAddressFull']['rawId']
            elif dictq[i] == q.queries['cr_store']:
                context['store_id'] = answer.json()['data']['createStore']['id']
                context['store_rawid'] = answer.json()['data']['createStore']['rawId']
            elif dictq[i] == q.queries['cr_warehouse']:
                context['war_id'] = answer.json()['data']['createWarehouse']['id']
            elif dictq[i] == q.queries['cr_b_prod']:
                context['b_prod_id'] = answer.json()['data']['createBaseProduct']['id']
                context['b_prod_rawid'] = answer.json()['data']['createBaseProduct']['rawId']
            elif dictq[i] == q.queries['cr_cust_attr']:
                context['cust_attr_rawid'] = answer.json()['data']['createCustomAttribute']['rawId']
            elif dictq[i] == q.queries['cr_prod']:
                context['prod_id'] = answer.json()['data']['createProduct']['id']
                context['prod_rawid'] = answer.json()['data']['createProduct']['rawId']
            elif dictq[i] == q.queries['cr_order']:
                context['order_slug'] = answer.json()['data']['createOrders']['invoice']['orders'][0]['slug']
            elif dictq[i] == q.queries['delete_fromcart']:
                token_headers['Authorization'] = ad_token
            print(answer.json())
            if 'errors' in answer.text:
                error_message = 'ЕСТЬ ОШИБКА В ЗАПРОСЕ: ' + str(i) + answer.text
                print (error_message)
                errors['message'+str(count)] = error_message
                count += 1
        except Exception as ex:
            errors['except'+str(count)] = 'ИСКЛЮЧЕНИЕ В ЗАПРОСЕ ' + i + '\n' + answer.text + '\n' + str(ex)
            print (errors['except'+str(count)])
            count += 1
        if len(errors) > 0:
            raise Exception(TestFailException)
    print('\n', '\n', 'ОШИБОК НЕ ОБНАРУЖЕНО')

action(q.queries)

