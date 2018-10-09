#!/usr/bin/env python3

#Это словарь со всеми запросами для GaphQL
queries = {

'version' : '''{"query":
                "query {apiVersion}"}''',

'languages' : '''{"query":
                    "query {languages{isoCode}}"}''',

'currencies' : '''{"query":
                    "query {currencies}"}''',

'categories' : '''{"query":
                    "query {categories{id, rawId, name{text}, level, parentId}}"}''',

'currencyExchange' : '''{"query":
                            "query {currencyExchange{code, rates{code, value}}}"}''',

'adm_token' : '''{
    "query":
        "mutation getJWTByEmail($input: CreateJWTEmailInput!) {getJWTByEmail (input: $input) {token}}",
    "variables": {
        "input": {
            "clientMutationId": "1",
            "email": "admin@storiqa.com",
            "password": "bqF5BkdsCS"
        }
    }
}''',

'cr_cat1' : '''
{"query":
    "mutation createCategory($input: CreateCategoryInput!) {createCategory(input: $input) {id rawId name {lang text}}}",
    "variables": 
        {"input": {
            "clientMutationId": "1",
            "name": [
                {"lang": "DE", "text": "test"},
                {"lang": "RU", "text": "тест%(n)s"}
                ],
            "parentId": 0
        }
    }
}
''',

'cr_cat2' : '''
{"query":
    "mutation createCategory($input: CreateCategoryInput!) {createCategory(input: $input) {id rawId name {lang text}}}",
    "variables": 
        {"input": {
            "clientMutationId": "1",
            "name": [
                {"lang": "DE", "text": "test"},
                {"lang": "RU", "text": "тест%(n)s"}
                ],
            "parentId": %(cat_rawid_1)i
        }
    }
}
''',

'cr_cat3' : '''
{"query":
    "mutation createCategory($input: CreateCategoryInput!) {createCategory(input: $input) {id rawId name {lang text}}}",
    "variables": 
        {"input": {
            "clientMutationId": "1",
            "name": [
                {"lang": "DE", "text": "test"},
                {"lang": "RU", "text": "тест%(n)s"}
                ],
            "parentId": %(cat_rawid_2)i
        }
    }
}
''',

'up_cat' : """
{"query":
    "mutation updateCategory($input: UpdateCategoryInput!) {updateCategory(input: $input) {id}}",
    "variables": {
        "input" : {
            "id": "%(cat_id_3)s",
            "clientMutationId": "1",
            "name": [{"lang": "EN", "text": "test%(n)s"}]
        }
    } 
}
""",

'cr_attr' : '''
{"query":
    "mutation createAttribute($input: CreateAttributeInput!) {createAttribute(input: $input) {id, rawId}}",
    "variables": {
        "input" : {
        "clientMutationId": "1",
        "name": [{"text": "test", "lang": "EN"}],
        "valueType": "STR",
        "metaField": {
            "values": ["test"],
            "uiElement": "COMBOBOX"
            }
        }
    }
}
''',

'up_attr' : '''
{"query":
    "mutation updateAttribute($input: UpdateAttributeInput!) {updateAttribute(input: $input) {id}}",
    "variables": {
        "input" : {
            "clientMutationId": "1",
            "id":  "%(attr_id)s",
            "name": [{"text": "test%(n)s", "lang": "EN"}]
        }
    }
}
''',

'add_attr' : '''
{"query":
    "mutation addAttributeToCategory($input: AddAttributeToCategoryInput!) {addAttributeToCategory(input: $input) {mock}}",
    "variables": {
        "input": {
            "clientMutationId": "1",
            "catId": %(cat_rawid_3)i,
            "attrId": %(attr_rawid)i
        }
    }
}
''',

'cr_company' : '''
{"query":
    "mutation createCompany($input: NewCompanyInput!) {createCompany (input: $input) {id, rawId}}",
    "variables": {
         "input": {
            "clientMutationId": "1",
            "name": "testCompany",
            "label": "TST",
            "description": "company for tests",
            "deliveriesFrom": ["LBR"],
            "logo": "TST"
        }
    }
}
''',

'up_company' : '''
{"query":
    "mutation updateCompany($input: UpdateCompanyInput!) {updateCompany (input: $input) {deliveriesFrom {alpha3 children{alpha3 children{alpha3}}}}}",
    "variables": {
         "input": {
            "clientMutationId": "1",
			"id": "%(company_id)s",
            "deliveriesFrom": ["LBR", "RUS"],
            "logo": "XxX_TST_XxX"
        }
    }
}
''',

'cr_package' : '''
{"query":
    "mutation createPackage($input: NewPackagesInput!) {createPackage (input: $input) {id, rawId}}",
    "variables": {
        "input": {
            "clientMutationId": "1",
            "name": "testPackge",
            "maxSize": 10.0,
            "minSize": 1.0,
            "maxWeight": 30.0,
            "minWeight": 0.5,
            "deliveriesTo": ["LBR"]
        }
    }
}
''',

'up_package' : '''
{"query":
    "mutation updatePackage($input: UpdatePackagesInput!) {updatePackage (input: $input) {deliveriesTo {alpha3 children{alpha3 children{alpha3}}}}}",
    "variables": {
        "input": {
            "clientMutationId": "1",
            "id": "%(package_id)s",
            "maxSize": 11.0,
            "minSize": 1.5,
            "maxWeight": 31.0,
            "minWeight": 0.8,
            "deliveriesTo": ["LBR", "RUS"]
        }
    }
}
''',

'ad_package' : '''
{"query":
    "mutation addPackageToCompany($input: NewCompaniesPackagesInput!) {addPackageToCompany (input: $input) {id, rawId}}",
    "variables": {
         "input": {
            "clientMutationId": "1",
			"companyId": %(company_rawid)i,
          	"packageId": %(package_rawid)i 
        }
    }
}
''',



'cr_user' : '''
{"query":
	"mutation createUser($input: CreateUserInput!) {createUser(input: $input) {id rawId}}",
	"variables": {
	    "input": {
	        "clientMutationId": "1",
	        "firstName": "tester",
	        "lastName": "testoviy",
	        "email": "%(regmail)s",
	        "password": "qwe123QWE" 
	    }
	},
	"operationName": "createUser"
}
''',

'ad_role_user_users' : '''
{"query":
    "mutation addRoleToUserOnUsersMicroservice($input: NewUsersRoleInput!) {addRoleToUserOnUsersMicroservice (input: $input) {userId, name}}",
    "variables": {
        "input": {
            "clientMutationId": "1",
            "userId": %(n_usr_rawid)i,
            "name": "MODERATOR"
        }
    }
}
''',

'ad_role_user_stores' : '''
{"query":
    "mutation addRoleToUserOnStoresMicroservice($input: NewStoresRoleInput!) {addRoleToUserOnStoresMicroservice (input: $input) {userId, name}}",
    "variables": {
        "input": {
            "clientMutationId": "1",
            "userId": %(n_usr_rawid)i,
            "name": "MODERATOR"
        }
    }
}
''',

'del_role_user_users' : '''
{"query":
    "mutation removeRoleFromUserOnUsersMicroservice($input: RemoveUsersRoleInput!) {removeRoleFromUserOnUsersMicroservice (input: $input) {userId, name}}",
    "variables": {
        "input": {
            "clientMutationId": "1",
            "userId": %(n_usr_rawid)i,
            "name": "MODERATOR"
        }
    }
}
''',

'del_role_user_stores' : '''
{"query":
    "mutation removeRoleFromUserOnStoresMicroservice($input: RemoveStoresRoleInput!) {removeRoleFromUserOnStoresMicroservice (input: $input) {userId, name}}",
    "variables": {
        "input": {
            "clientMutationId": "1",
            "userId": %(n_usr_rawid)i,
            "name": "MODERATOR"
        }
    }
}
''',

'user_token' : '''
{"query":
	"mutation getJWTByEmail($input: CreateJWTEmailInput!) {getJWTByEmail (input: $input) {token}}",
	"variables": {
	    "input": {
	        "clientMutationId": "1",
	        "email": "tester@storiqa.com",
	        "password": "qwe123QWE"
	    }
	}
}
''',

'user_id' : '''
{"query":
	"query {me {id, rawId, isActive, myStore {id}}}" }
''',

'up_user' : '''
{"query":
    "mutation updateUser($input: UpdateUserInput!) {updateUser(input: $input){id, isActive}}",
    "variables": {
        "input": {
            "clientMutationId": "1",
            "id": "%(usr_id)s",
            "phone": "89095754585",
            "firstName": "Testoviy",
            "lastName": "User%(n)s",
            "middleName": "epta",
            "gender": "MALE",
            "birthdate": "1987-04-04"
        }
    }
}
''',

'createUserDeliveryAddressFull' : '''
{"query":
    "mutation createUserDeliveryAddressFull($input: NewUserDeliveryAddressFullInput!) {createUserDeliveryAddressFull(input: $input) {id, rawId, isPriority}}",
    "variables": {
        "input" : {
            "clientMutationId": "1",
            "userId": %(usr_rawid)i,
            "addressFull" : {"country": "United States", "postalCode": "432234"},
            "isPriority": true
        }
    }
}
''',

'updateUserDeliveryAddressFull' : '''
{"query":
    "mutation updateUserDeliveryAddressFull($input: UpdateUserDeliveryAddressFullInput!) {updateUserDeliveryAddressFull(input: $input) {id, rawId, isPriority}}",
    "variables": {
        "input" : {
            "clientMutationId": "1",
            "id": %(addr_rawid)i,
            "addressFull" : {"value": "kakayato avenue 34", "country": "United States", "postalCode": "432234"},
            "isPriority": true
        }
    }
}
''',

'deleteUserDeliveryAddressFull' : '''
{"query":
    "mutation deleteUserDeliveryAddressFull {deleteUserDeliveryAddressFull(id: %(addr_rawid)i) {id, rawId}}"
}
''',

'cr_wizard' : '''
{"query":
    "mutation createWizardStore {createWizardStore {id, storeId}}"
}
''',

'up_wizard' : '''
{"query":
    "mutation updateWizardStore($input: UpdateWizardStoreInput!) {updateWizardStore(input: $input) {id, storeId}}",    
    "variables": {
        "input" : {
            "clientMutationId": "1",
          	"addressFull": {"country": "Russia", "postalCode": "123321"}
        }
    }
}
''',

'del_wizard' : '''
{"query":
    "mutation deleteWizardStore {deleteWizardStore {id, storeId}}"
}
''',

'cr_store' : '''
{"query":
    "mutation createStore($input: CreateStoreInput!) {createStore(input: $input) {id, name{lang, text}, rawId}}",
    "variables": {
        "input": {
            "clientMutationId": "1",
            "name": [{"lang": "EN", "text": "testshop%(n)s"}],
            "userId": %(usr_rawid)i,
            "defaultLanguage": "EN",
            "shortDescription": [{"lang": "EN", "text": "test"}],
            "slug": "test%(n)s",
            "phone": "89091112233",
            "email": "teststore@test.test",
            "addressFull": {"value": "gdeto", "country": "Canada", "postalCode": "111111"}
        }
    }
}
''',

'up_store' : '''
{"query":
    "mutation updateStore($input: UpdateStoreInput!) {updateStore(input: $input) {id, isActive}}",
    "variables": {
        "input": {
            "clientMutationId": "1",
            "id": "%(store_id)s",
            "name": [{"lang": "EN", "text": "qwerty%(n)s"}],
            "shortDescription": [{"lang": "EN", "text": "short"}],
            "longDescription": [{"lang": "EN", "text": "long"}],
            "slug": "xxx%(n)s",
            "phone": "89093335522",
            "email": "ssss@test.test",
            "addressFull": {"value": "tamto", "country": "Canada", "postalCode": "111222"}
            
        }
    }
}
''',

'cr_warehouse' : '''
{"query":
    "mutation createWarehouse($input: CreateWarehouseInput!) {createWarehouse (input: $input) {id}}",
    "variables": {
        "input": {
            "clientMutationId": "1",
            "name": "testwar",
            "slug": "testwar",
            "storeId": %(store_rawid)i,
            "addressFull": {"value": "gdeto", "country": "Liberia", "postalCode": "111111", "countryCode": "LBR"}
        }
    }
}
''',

'up_warehouse' : '''
{"query":
    "mutation updateWarehouse($input: UpdateWarehouseInput!) {updateWarehouse (input: $input) {id, name}}",
    "variables": {
        "input": {
            "clientMutationId": "1",
            "name": "testwarE",
            "slug": "testwar%(n)s",
            "id": "%(war_id)s",
            "addressFull": {"value": "gdeto", "country": "Canada", "postalCode": "111111"}
        }
    }
}
''',

'cr_b_prod' : '''
{"query":
    "mutation createBaseProduct($input: CreateBaseProductInput!) {createBaseProduct(input: $input) {id rawId name {lang text}}}",
    "variables": {
        "input" : {
            "clientMutationId": "1",
             "name": [{"lang": "EN", "text": "testproduct%(n)s"},
                      {"lang": "RU", "text": "тестпродукт%(n)s"}],
             "storeId": %(store_rawid)i,
             "currency": "STQ",
             "categoryId": %(cat_rawid_3)i,
             "slug": "bptest%(n)s",
             "shortDescription": [{"lang": "EN", "text": "test"}]
        }
    }
}
''',

'up_b_prod' : '''
{"query":
    "mutation updateBaseProduct($input: UpdateBaseProductInput!) {updateBaseProduct(input: $input) {id, rawId}}",
    "variables": {
        "input" : {
            "clientMutationId": "1",
             "id": "%(b_prod_id)s",
             "longDescription" : [{"lang" : "EN", "text" : "Long Desc"}]
        }
    }
}
''',

'cr_cust_attr' : '''
{"query":
    "mutation createCustomAttribute($input: NewCustomAttributeInput!) {createCustomAttribute (input: $input) {id, rawId}}",
    "variables": {
        "input":{
            "clientMutationId": "1",
			"attributeId": %(attr_rawid)i,
            "baseProductId": %(b_prod_rawid)i
        }
    }
}
''',

'cr_prod' : '''
{"query":
    "mutation createProduct($input: CreateProductWithAttributesInput!) {createProduct(input: $input) {id, isActive, rawId}}",
    "variables": {
        "input" : {
            "clientMutationId": "1",
             "product":  {"baseProductId": %(b_prod_rawid)i,
                          "preOrder": true,
                          "preOrderDays": 10,
                          "price": 200.00,
                          "vendorCode": "11"},
             "attributes": [{"attrId": 1,
                             "value": "1",
                             "metaField": "dfasfas"}]   
        }
    }
}
''',

'up_prod' : '''
{"query":
    "mutation updateProduct($input: UpdateProductWithAttributesInput!) {updateProduct(input: $input) {id}}",
    "variables": {
        "input": {
            "clientMutationId": "1",
            "id": "%(prod_id)s",
            "product": {"discount": 1.0},
            "customAttributes": [{
                     "customAttributeId": %(cust_attr_rawid)i,
                     "value": "test"}]
        }
    }
}
''',

'upsert_ship' : '''
{"query":
    "mutation upsertShipping($input: NewShippingInput!) {upsertShipping (input: $input) {local {deliveriesTo {alpha3 children{alpha3 children{alpha3}}}}}}",
    "variables": {
        "input": {
            "clientMutationId": "1",
            "local": {"companyPackageId": %(comp_pack_rawid)i, "price": 100.0},
            "international": {"companyPackageId": %(comp_pack_rawid)i, "price": 200.0, "deliveriesTo": ["LBR"]},
            "pickup": {"pickup": true, "price": 0.0},
            "baseProductId": %(b_prod_rawid)i,
			"storeId": %(store_rawid)i
        }
    }
}
''',

'get_b_prods' : '''
{"query":
    "{me {baseProducts {edges {node {id rawId}}}}}"
}
''',

'prod_in_war' : '''
{"query":
    "mutation setProductQuantityInWarehouse($input: ProductQuantityInput!) {setProductQuantityInWarehouse (input: $input) {id, productId, quantity}}",
    "variables": {
        "input":{
            "clientMutationId": "1",
            "warehouseId": "%(war_id)s",
            "productId": %(prod_rawid)i,
            "quantity": 33
        }
    }
}
''',

'publish_store' : '''
{"query":
    "mutation publishStore {publishStore(id: %(store_rawid)i) {id}}"
}
''',

'publish_b_prod' : '''
{"query":
    "mutation publishBaseProducts {publishBaseProducts(ids: %(b_prod_rawid)i) {id}}"
}
''',

'clear_Cart' : '''
{"query":
    "mutation clearCart  {clearCart{id, totalCost}}"}
''',

'increment_incart' : '''
{"query":
    "mutation incrementInCart($input: IncrementInCartInput!) {incrementInCart(input: $input) {id, productsCost}}",
"variables": {
    "input": {
        "clientMutationId": "1",
        "productId": %(prod_rawid)i
    }
 }
}
''',

'setselection_incart' : '''
{"query":
    "mutation setSelectionInCart($input: SetSelectionInCartInput!) {setSelectionInCart(input: $input) {id, productsCost}}",
"variables": {
    "input": {
        "clientMutationId": "1",
        "productId": %(prod_rawid)i,
        "value": true
    }
 }}
''',

'setquantity_incart' : '''
{"query":
    "mutation setQuantityInCart($input: SetQuantityInCartInput!) {setQuantityInCart(input: $input) {id, productsCost}}",
"variables": {
    "input": {
        "clientMutationId": "1",
        "productId": %(prod_rawid)i,
        "value": 3
    }
 }}
''',

'cr_order' : '''
{"query":
    "mutation createOrders($input: CreateOrderInput!) {createOrders (input: $input) {invoice{id, orders{id, slug, trackId}}}}",
"variables": {
    "input": {
        "clientMutationId": "1",
        "addressFull": {"value": "gdeto", "country": "Canada", "postalCode": "111111"},
        "receiverName": "tester",
        "receiverPhone": "+79095623366",
        "currency": "STQ" 
    }
 }}
''',

'order_delivery' : '''
{"query":
    "mutation setOrderStatusDelivery($input: OrderStatusDeliveryInput!) {setOrderStatusDelivery (input: $input) {state, trackId}}",
"variables": {
    "input": {
        "clientMutationId": "1",
        "orderSlug": %(order_slug)i,
        "comment": "test"
    }
 }}
''',

'order_history' : '''
{"query":
    "query order_history {me{id, order(slug: %(order_slug)i){history {edges {node {state, orderId, comment}}}}}}"}
''',

'order_canceled' : '''
{"query":
    "mutation setOrderStatusCanceled($input: OrderStatusCanceledInput!) {setOrderStatusCanceled (input: $input) {state, trackId}}",
"variables": {
    "input": {
        "clientMutationId": "1",
        "orderSlug": %(order_slug)i,
        "comment": "test"
    }
 }}
''',

'order_complete' : '''
{"query":
    "mutation setOrderStatusComplete($input: OrderStatusCompleteInput!) {setOrderStatusComplete (input: $input) {state, trackId}}",
"variables": {
    "input": {
        "clientMutationId": "1",
        "orderSlug": %(order_slug)i,
        "comment": "test"
    }
 }}
''',

'delete_fromcart' : '''
{"query":
    "mutation deleteFromCart($input: DeleteFromCartInput!) {deleteFromCart(input: $input) {id, totalCost}}",
"variables": {
    "input": {
        "clientMutationId": "1",
        "productId": %(prod_rawid)i
    }
 }}
''',

'draft_b_prod' : '''
{"query":
    "mutation draftBaseProducts {draftBaseProducts(ids: %(b_prod_rawid)i) {id}}"
}
''',

'draft_store': '''
{"query":
    "mutation draftStore {draftStore(id: %(store_rawid)i) {id}}"
}
''',

'del_comp_pack' : '''
{"query":
    "mutation deleteCompanyPackage {deleteCompanyPackage(id: %(comp_pack_rawid)i) {id}}"
}
''',

'del_package' : '''
{"query":
    "mutation deletePackage {deletePackage(id: %(package_rawid)i) {id}}"
}
''',

'del_company' : '''
{"query":
    "mutation deleteCompany {deleteCompany(id: %(company_rawid)i) {id}}"
}
''',

# 'del_cust_attr' : '''
# {"query":
#     "mutation deleteCustomAttribute($input: DeleteCustomAttributeInput!) {deleteCustomAttribute (input: $input) {id, rawId}}",
#     "variables": {
#         "input":{
#             "clientMutationId": "1",
#             "customAttributeId": %(cust_attr_rawid)i
#         }
#     }
# }
# ''',

'del_attr_from_cat' : '''
{"query":
    "mutation deleteAttributeFromCategory($input: DeleteAttributeFromCategory!) {deleteAttributeFromCategory(input: $input) {mock}}",
    "variables": {
        "input": {
            "clientMutationId": "1",
            "catId": %(cat_rawid_3)i,
            "attrId": %(attr_rawid)i
        }
    }
}
''',

'deact_prod' : '''
{"query":
    "mutation deactivateProduct($input: DeactivateProductInput!) {deactivateProduct(input: $input) {id, isActive}}",
    "variables": {
        "input": {
            "clientMutationId": "1",
            "id": "%(prod_id)s"
        }
    }
}
''',

'deact_b_prod' : '''
{"query":
    "mutation deactivateBaseProduct($input: DeactivateBaseProductInput!) {deactivateBaseProduct(input: $input) {id, isActive}}",
    "variables": {
        "input": {
            "clientMutationId": "1",
            "id": "%(b_prod_id)s"
        }
    }
}
''',

'deact_store' : '''
{"query":
    "mutation deactivateStore($input: DeactivateStoreInput!) {deactivateStore(input: $input) {id, isActive}}",
    "variables": {
        "input":{
            "clientMutationId": "1",
            "id": "%(store_id)s"
        }
    }
}
'''
}



#Ниже не актуальные для использования в настоящее время запросы.

# deact_user : '''
# {"query":
#     "mutation deactivateUser($input:  DeactivateUserInput!) {deactivateUser(input: $input){id, isActive}}",
# "variables": {
#     "input": {
#         "clientMutationId": "1",
#         "id": "%(usr_id)s"
#     }
#  }}
# '''
# 'product_comment' : '''
# {"query":
#     "mutation createProductComment($input: CreateModeratorProductCommentsInput!) {createProductComment(input: $input) {id, comments}}",
# "variables": {
#     "input" : {
# 	    "clientMutationId": "1",
# 	    "moderatorId": 1,
#         "baseProductId": %(b_prod_rawid)i,
#         "comments": "asde"
#     }
# }}
# '''
#
# 'store_comment' : '''
# {"query":
#     "mutation createProductComment($input: CreateModeratorProductCommentsInput!) {createProductComment(input: $input) {id, comments}}",
# "variables": {
#     "input" : {
# 	    "clientMutationId": "1",
# 	    "moderatorId": 1,
#         "storeId": %(store_rawid)i,
#         "comments": "asde"
#     }
# }}
# '''
#
# 'del_warehouse' : '''
# {"query":
#     "mutation deleteWarehouse {deleteWarehouse(id: %(war_id)s) {id}}"
# }
# ''',
# 'createUserDeliveryAddress' : '''
# {"query":
#     "mutation createUserDeliveryAddress($input: NewUserDeliveryAddressInput!) {createUserDeliveryAddress(input: $input) {id, rawId, isPriority}}",
#     "variables": {
#         "input" : {
#             "clientMutationId": "1",
#             "userId": %(usr_rawid)i,
#             "country": "United States",
#             "postalCode": "432234",
#             "isPriority": true
#         }
#     }
# }
# ''',
#
# 'updateUserDeliveryAddress' : '''
# {"query":
#     "mutation updateUserDeliveryAddress($input: UpdateUserDeliveryAddressInput!) {updateUserDeliveryAddress(input: $input) {id, rawId, isPriority}}",
#     "variables": {
#         "input" : {
#             "clientMutationId": "1",
#             "id": %(addr_rawid)i,
#             "country": "Canada",
#             "postalCode": "436236",
#             "isPriority": true
#         }
#     }
# }
# ''',