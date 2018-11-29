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

errors:dict = {}

# Создает список из ключей словаря
def keys2list(anydict:dict):
    key_list = anydict.keys()
    key_list = list(key_list)
    return key_list

# Выводит пронумерованный столбец ключей
def list_colum(anylist:list):
    num = 0
    for i in anylist:
        print(num, i)
        num += 1

# Из большого словаря делает новый словарь по списку ключей
def select_query_part(keylist:list, anydict:dict):
    querypart = {}
    for i in keylist:
        querypart[i] = anydict[i]
    return querypart

# Пост запрос с параметрами
def request(json_query, headers, cookies):
    r = requests.post(url, json=json_query, headers=headers, cookies=cookies)
    return r

# Словарь с переменными использующимися в запросах
context:dict = {
    'n': datetime.strftime(datetime.now(), "%m%d%H%M%S"),
    'adm' : 'admin@storiqa.com',
    'admpwd' : 'bqF5BkdsCS',
    'usr' : 'apitester@storiqa.com',
    'usrpwd' : 'qwe123QWE',
    'cat_rawid_1' : '',
    'cat_rawid_2' : '',
    'cat_rawid_3' : '',
    'cat_id_3' : '',
    'attr_id' : '',
    'attr_rawid' : '',
    'company_id' : '',
    'company_rawid' : '',
    'package_id' : '',
    'package_rawid' : '',
    'comp_pack_rawid' : '',
    'new_usr_rawid' : '',
    'usr_id' : '',
    'usr_rawid' : '',
    'addr_id' : '',
    'addr_rawid' : '',
    'store_id' : '',
    'store_rawid' : '',
    'war_id' : '',
    'b_prod_id' : '',
    'b_prod_rawid' : '',
    'cust_attr_rawid' : '',
    'prod_id' : '',
    'prod_rawid' : '',
    'shipping_id' : '',
    'coupon_rawid' : '',
    'coupon_code' : '',
    'order_slug' : '',
    'invoice_id' : ''
}
# Действие со списком запросов. Основная логика теста.
def action(dictq:dict):
    token_headers:dict = {"currency" : "STQ"}
    cookie:dict = {"holyshit": "iamcool"}
    answer:json
    count:int = 0
    context['regmail'] = 'test' + context['n'] + '@test.test'
    for i in dictq:
        try:
            answer = request(json.loads(dictq[i] % context), token_headers, cookie)
            if dictq[i] == q.queries['admin_getJWTByEmail']:
                ad_token = 'Bearer ' + answer.json()['data']['getJWTByEmail']['token']
                token_headers['Authorization'] =  ad_token
            elif dictq[i] == q.queries['createCategory_1']:
                context['cat_id_1'] = answer.json()['data']['createCategory']['id']
                context['cat_rawid_1'] = answer.json()['data']['createCategory']['rawId']
            elif dictq[i] == q.queries['createCategory_2']:
                context['cat_id_2'] = answer.json()['data']['createCategory']['id']
                context['cat_rawid_2'] = answer.json()['data']['createCategory']['rawId']
            elif dictq[i] == q.queries['createCategory_3']:
                context['cat_id_3'] = answer.json()['data']['createCategory']['id']
                context['cat_rawid_3'] = answer.json()['data']['createCategory']['rawId']
            elif dictq[i] == q.queries['createAttribute']:
                context['attr_id'] = answer.json()['data']['createAttribute']['id']
                context['attr_rawid'] = answer.json()['data']['createAttribute']['rawId']
            elif dictq[i] == q.queries['createCompany']:
                context['company_id'] = answer.json()['data']['createCompany']['id']
                context['company_rawid'] = answer.json()['data']['createCompany']['rawId']
            elif dictq[i] == q.queries['createPackage']:
                context['package_id'] = answer.json()['data']['createPackage']['id']
                context['package_rawid'] = answer.json()['data']['createPackage']['rawId']
            elif dictq[i] == q.queries['addPackageToCompany']:
                context['comp_pack_rawid'] = answer.json()['data']['addPackageToCompany']['rawId']
            elif dictq[i] == q.queries['createUser']:
                context['new_usr_rawid'] = answer.json()['data']['createUser']['rawId']
            elif dictq[i] == q.queries['user_getJWTByEmail']:
                token = 'Bearer ' +  answer.json()['data']['getJWTByEmail']['token']
                token_headers['Authorization'] = token
            elif dictq[i] == q.queries['query_me']:
                context['usr_id'] = answer.json()['data']['me']['id']
                context['usr_rawid'] = answer.json()['data']['me']['rawId']
                try:
                    context['store_id'] = answer.json()['data']['me']['myStore']['id']
                    if len(context['store_id']) > 0:
                        request(json.loads(q.queries['deactivateStore'] % context), token_headers, cookie)
                except: TypeError()
            elif dictq[i] == q.queries['createUserDeliveryAddressFull']:
                context['addr_id'] = answer.json()['data']['createUserDeliveryAddressFull']['id']
                context['addr_rawid'] = answer.json()['data']['createUserDeliveryAddressFull']['rawId']
            elif dictq[i] == q.queries['createStore']:
                context['store_id'] = answer.json()['data']['createStore']['id']
                context['store_rawid'] = answer.json()['data']['createStore']['rawId']
            elif dictq[i] == q.queries['createWarehouse']:
                context['war_id'] = answer.json()['data']['createWarehouse']['id']
            elif dictq[i] == q.queries['createBaseProduct']:
                context['b_prod_id'] = answer.json()['data']['createBaseProduct']['id']
                context['b_prod_rawid'] = answer.json()['data']['createBaseProduct']['rawId']
            elif dictq[i] == q.queries['createCustomAttribute']:
                context['cust_attr_rawid'] = answer.json()['data']['createCustomAttribute']['rawId']
            elif dictq[i] == q.queries['createProduct']:
                context['prod_id'] = answer.json()['data']['createProduct']['id']
                context['prod_rawid'] = answer.json()['data']['createProduct']['rawId']
            elif dictq[i] == q.queries['createCoupon']:
                context['coupon_rawid'] = answer.json()['data']['createCoupon']['rawId']
                context['coupon_code'] = answer.json()['data']['createCoupon']['code']
            elif dictq[i] == q.queries['availableShippingForUser']:
                context['shipping_id'] = answer.json()['data']['availableShippingForUser']['packages'][0]['shippingId']
            elif dictq[i] == q.queries['createOrders']:
                context['order_slug'] = answer.json()['data']['createOrders']['invoice']['orders'][0]['slug']
            elif dictq[i] == q.queries['buyNow']:
                context['invoice_id'] = answer.json()['data']['buyNow']['invoice']['id']
            elif dictq[i] == q.queries['deleteFromCart']:
                token_headers['Authorization'] = ad_token
            print(answer.json())
            if 'errors' in answer.text:
                error_message = 'ERROR IN QUERY: ' + str(i) + answer.text
                print (error_message)
                errors['message'+str(count)] = error_message
                count += 1
        except Exception as ex:
            errors['except'+str(count)] = 'EXCEPTION IN QUERY: ' + i + '\n' + str(ex)
            print (errors['except'+str(count)])
            count += 1
        if len(errors) > 0:
            raise Exception(TestFailException)
    print('\n', '\n', 'Yay! All processes COMPLETED. Bakend developers well done ^_^')

'''
Чтобы протестировать только некоторые запросы нужно установить значение test_all=0,
в переменной list_indexes задать индексы ключей к запросам, которые необходимо выполнить,
чтобы вывести индексы ключей к запросам нужно установить значение print_keylist=1.
Чтобы задать необходимые переменные нужно написать их значение в словарь context в строке 44
'''
test_all:bool = 1
print_keylist:bool = 0

if test_all == 1:
    action(q.queries)
else:
    if print_keylist == 1:
        list_colum(keys2list(q.queries)) # Выполнить чтобы увидеть список ключей для выбора
    else:
        u = 23  # Индекс запроса для получения токена admin = 5, user = 23
        a = 5
        list_indexes = [a, 6, 7, 8, 10, u, 24, 32, 34, 36, 39, 44, 45, 46, 47, 48] # Указать нужные ключи
        keylist = keys2list(q.queries)
        actual_keys:list = []
        for n in list_indexes:
            actual_keys.append(keylist[n])
        querypart = select_query_part(actual_keys, q.queries)
        action(querypart)





