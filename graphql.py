# -*- coding: utf-8 -*-
import requests
import json

#x = raw_input("Insert test number: ")
#n = str(x)
url = 'http://nightly.stq.cloud:60088/graphql'

def request(b, c):
	r = requests.post(url, json=b, headers=c)
	return r
	
#Проверка версии	
version = {"query": "query {apiVersion}"}
request_version = request(version, '')
print request_version.text

#Запрос сброса пароля
pass_reset = {"query": "mutation requestPasswordReset($input:  ResetRequest!) {requestPasswordReset(input: $input) {success}}",
	"variables": {"input": {"clientMutationId": "1",
		"email": "tester16@test.test"}} }
get_pass_reset = (pass_reset, '')
print get_pass_reset.text

#Отправка подтверждения емейл
send_email = {"query": "mutation resendEmailVerificationLink($input:  VerifyEmailResend!) {resendEmailVerificationLink(input: $input) {success}}",
	"variables": {"input": {"clientMutationId": "1",
		"email": "tester1@test.test"}} }
get_send_email = (send_email, '')
print get_send_email.text
