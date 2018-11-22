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

'aval_ship' : '''
{"query":
    "{availableShippingForUser(userCountry: "RUS", baseProductId: 1874) {packages {id shippingId}}}"
'''
}


context = {
        'n': datetime.strftime(datetime.now(), "%m%d%H%M%S"),
        'war_id' : 'id: 92528575-c0e9-4fa8-9d42-f85d88e3ed6e'
    }

cookie = {"holyshit": "iamcool"}
theaders = {"currency" : "STQ", "Authorization" : "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJ1c2VyX2lkIjoxOTcsImV4cCI6MTU0MjI3MjcwNywicHJvdmlkZXIiOiJFbWFpbCJ9.CfSd4aDPWQdi1rqpPae1axQ8ARXX0CYQCT8DhBQJ020WV0OUjH28UVpLYOHYpBF_NFhH8AE501z-gVh6wXYET9sDdO7PC4ZOD2IuSCDVBA2gVgyeD2cUrRVZIBPE4d8BUZyF4i3uv227cInsWDX3MG-pyDP-4waRAGtzsTfIHnIMNaeMUzTrmxXA5e3L11GIX_IwpFYmEpBTNK7lNn4cpBXlkAvEhC7KN1BOhXwv_ul2fe4-kqWzLdy2NU1QgqXev1B8WhIiu2zu0IxpR7nIDzydrEtfNAYdIz35p3_Vw4ofHTXKg3vKh9bfmT_EK_380vZ4kbHIwyM-PtzH2G3jrA"}
def request(json_query, headers, cookies):
    url = 'https://nightly.stq.cloud/graphql'
    r = requests.post(url, json=json_query, headers=headers, cookies=cookies)
    return r

rus = "RUS"
answer = request(json.loads('''{"query" : "{availableShippingForUser(userCountry: \\"RUS\\", baseProductId: 1874) {packages {id shippingId}}}"}'''), theaders, cookie)
print(answer.json())
