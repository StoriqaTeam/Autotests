# -*- coding: utf-8 -*-
import requests
import json
from datetime import datetime

query = {
# 'user_token' : '''
# {"query":
# 	"mutation getJWTByEmail($input: CreateJWTEmailInput!) {getJWTByEmail (input: $input) {token}}",
# 	"variables": {
# 	    "input": {
# 	        "clientMutationId": "1",
# 	        "email": "apitester@storiqa.com",
# 	        "password": "qwe123QWE"
# 	    }
# 	}
# }
# ''',

'user_id' : '''
{"query":
	"query {me {id, rawId, isActive, myStore {id}}}" }
''',

'del_warehouse' : '''
{"query":
    "mutation deleteWarehouse {deleteWarehouse(%(war_id)s) {id}}"
}
'''
}


context = {
        'n': datetime.strftime(datetime.now(), "%m%d%H%M%S"),
        'war_id' : 'id: 92528575-c0e9-4fa8-9d42-f85d88e3ed6e'
    }

cookie = {"holyshit": "iamcool"}
theaders = {"currency" : "STQ", "Authorization" : "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJ1c2VyX2lkIjoxOTcsImV4cCI6MTUzNzQ0NzYyOSwicHJvdmlkZXIiOiJFbWFpbCJ9.gbKvRHYGgKhStq-8F7W_CW7IakP18e5wFteOvIIhxFlUe8z_J7t6KI_0zoW8BokX6moF5qnAGIVOgKLO7k9E0x6NTFpgOlSytwSbswouIv9aYgjwI_jVzU5MZObe_RIY0M796wE4wXy8SDzrG6O9YDwY1-ihAZX0qyqxywkNi_skZ0r96lehcJwBhqDcphitubvpGBpYCWjGb92Ck104DPwJ7uUPGK64MX2FAAzA5eYJlO9118BLlw_6oC6bw1Hsc5fbG8_nmxEdEE3HkVnOPXLFwlismk_VnOKwsCcdCTO-0tLYwNbgbx9rr6TS2L4on065ikRP04E75iMavoCtMw"}
def request(json_query, headers, cookies):
    url = 'https://nightly.stq.cloud/graphql'
    r = requests.post(url, json=json_query, headers=headers, cookies=cookies)
    return r
for i in query:
    answer = request(json.loads(query[i] % context), theaders, cookie)
    print(answer.json())
