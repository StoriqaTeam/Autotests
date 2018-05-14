#!/usr/bin/env python3

import requests
import json
import os

url = os.environ['GRAPHQL_URL']

#Проверка версии	
version = {"query": "query {apiVersion}"}

#Запрос сброса пароля
pass_reset = {
    "query": """mutation requestPasswordReset($input: ResetRequest!) {
        requestPasswordReset(input: $input) {
            success
        }
    }""",
    "variables": {
        "input": {
            "clientMutationId": "1",
            "email": "tester16@test.test"
        }
    }
}

#Отправка подтверждения емейл
send_email = {
    "query": """mutation resendEmailVerificationLink($input: VerifyEmailResend!) {
        resendEmailVerificationLink(input: $input) {
            success
        }
    }""",
    "variables": {
        "input": {
            "clientMutationId": "1",
            "email": "tester1@test.test"
        }
    }
}

if __name__ == '__main__':
    def request(b, c):
        r = requests.post(url, json=b, headers=c)
        return r

    request_version = request(version, '')
    print(request_version)

    get_pass_reset = (pass_reset, '')
    print(get_pass_reset)

    get_send_email = (send_email, '')
    print(get_send_email)
