# -*- coding: utf-8 -*-
import requests
import json


#n = datetime.strftime(datetime.now(), "%m.%d_%H%M%S")
#regmail = 'test' + n + '@test.test'
x = raw_input("Insert test number: ")
n = str(x)
url = 'http://nightly.stq.cloud:60088/graphql'

def request(b, c):
	r = requests.post(url, json=b, headers=c)
	return r
	
#Проверка версии	
version = {"query": "query {apiVersion}"}
request_version = request(version, '')
print request_version.text

#Получение токена админа
admin_token = {"query": "mutation getJWTByEmail($input: CreateJWTEmailInput!) {getJWTByEmail (input: $input) {token}}",
	"variables": {"input": {"clientMutationId": "1", 
		"email": "admin@storiqa.com", 
		"password": "bqF5BkdsCS" }} }
get_admin_token = request(admin_token, '')
print get_admin_token.text
token = get_admin_token.json()['data']['getJWTByEmail']['token']
token_admin = {'Authorization': 'Bearer '+token}
print 'Admin token is: %s' % (token)

#Создание пользователя
user = {"query": "mutation createUser($input: CreateUserInput!) {createUser(input: $input) {id}}",
	"variables": {"input": {"clientMutationId": "1", 
		#"email": "test%s@test.test"%(n),
		"email": "tester@storiqa.com",
		#"email": regmail,
		"password": "qwe123QWE+" }},"operationName": "createUser"}
create_user = request(user, '')
print create_user.text

#Получение токена пользователя
user_token = {"query": "mutation getJWTByEmail($input: CreateJWTEmailInput!) {getJWTByEmail (input: $input) {token}}",
	"variables": {"input": {"clientMutationId": "1", 
		"email": "tester@storiqa.com", 
		"password": "qwe123QWE+" }} }
get_user_token = request(user_token, '')
token = get_user_token.json()['data']['getJWTByEmail']['token']
print 'User token is: %s' % (token)
token_headers = {'Authorization': 'Bearer '+token}

#Получаение ID пользователя
user_id = {"query": "query {me {id, rawId, isActive}}"}
get_user_id = request(user_id, token_headers)
print get_user_id.text
id = get_user_id.json()['data']['me']['id']
rawId = get_user_id.json()['data']['me']['rawId']

#Редактирование пользователя
update_user = {"query": "mutation updateUser($input:  UpdateUserInput!) {updateUser(input: $input){id, isActive}}",
	"variables": {"input": {"clientMutationId": "1", 
		"id": id, 
		"phone": "89095754585", 
		"firstName": "Testoviy", 
		"lastName": "User", 
		"middleName": "epta", 
		"gender": "MALE", 
		"birthdate": "1987-04-04" }} }
get_update_user = request(update_user, token_headers)
print get_update_user.text

#Выключение пользователя
''' deactivate_user = {"query": "mutation deactivateUser($input:  DeactivateUserInput!) {deactivateUser(input: $input){id, isActive}}",
	"variables": {"input" :{"clientMutationId": "1", "id": id}}  }
get_deactivate_user = request(deactivate_user, token_headers)
print get_deactivate_user, get_deactivate_user.text '''