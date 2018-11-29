#!/usr/bin/env python3

#Это словарь со всеми запросами для GaphQL
from typing import Dict, Any, Union

queries: Dict[Union[str, Any], Union[str, Any]] = {

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

'admin_getJWTByEmail' : '''{
    "query":
        "mutation getJWTByEmail($input: CreateJWTEmailInput!) {getJWTByEmail (input: $input) {token}}",
    "variables": {
        "input": {
            "clientMutationId": "1",
            "email": "%(adm)s",
            "password": "%(admpwd)s"
        }
    }
}''',

'createCategory_1' : '''
{"query":
    "mutation createCategory($input: CreateCategoryInput!) {createCategory(input: $input) {id rawId name {lang text}}}",
    "variables": 
        {"input": {
            "clientMutationId": "",
            "name": [
                {"lang": "EN", "text": "test"},
                {"lang": "RU", "text": "тест%(n)s"}
                ],
            "parentId": 0
        }
    }
}
''',

'createCategory_2' : '''
{"query":
    "mutation createCategory($input: CreateCategoryInput!) {createCategory(input: $input) {id rawId name {lang text}}}",
    "variables": 
        {"input": {
            "clientMutationId": "",
            "name": [
                {"lang": "DE", "text": "test"},
                {"lang": "RU", "text": "тест%(n)s"}
                ],
            "parentId": %(cat_rawid_1)i
        }
    }
}
''',

'createCategory_3' : '''
{"query":
    "mutation createCategory($input: CreateCategoryInput!) {createCategory(input: $input) {id rawId name {lang text}}}",
    "variables": 
        {"input": {
            "clientMutationId": "",
            "name": [
                {"lang": "DE", "text": "test"},
                {"lang": "RU", "text": "тест%(n)s"}
                ],
            "parentId": %(cat_rawid_2)i
        }
    }
}
''',

'updateCategory' : """
{"query":
    "mutation updateCategory($input: UpdateCategoryInput!) {updateCategory(input: $input) {id}}",
    "variables": {
        "input" : {
            "id": "%(cat_id_3)s",
            "clientMutationId": "",
            "name": [{"lang": "EN", "text": "test%(n)s"}]
        }
    } 
}
""",

'createAttribute' : '''
{"query":
    "mutation createAttribute($input: CreateAttributeInput!) {createAttribute(input: $input) {id, rawId}}",
    "variables": {
        "input" : {
        "clientMutationId": "",
        "name": [{"text": "test", "lang": "EN"}],
        "valueType": "STR",
        "metaField": {
            "values": ["test1", "test2"],
            "uiElement": "COMBOBOX"
            }
        }
    }
}
''',

'updateAttribute' : '''
{"query":
    "mutation updateAttribute($input: UpdateAttributeInput!) {updateAttribute(input: $input) {id}}",
    "variables": {
        "input" : {
            "clientMutationId": "",
            "id":  "%(attr_id)s",
            "name": [{"text": "test%(n)s", "lang": "EN"}]
        }
    }
}
''',

'addAttributeToCategory' : '''
{"query":
    "mutation addAttributeToCategory($input: AddAttributeToCategoryInput!) {addAttributeToCategory(input: $input) {mock}}",
    "variables": {
        "input": {
            "clientMutationId": "",
            "catId": %(cat_rawid_3)i,
            "attrId": %(attr_rawid)i
        }
    }
}
''',

'createCompany' : '''
{"query":
    "mutation createCompany($input: NewCompanyInput!) {createCompany (input: $input) {id, rawId}}",
    "variables": {
         "input": {
            "clientMutationId": "",
            "name": "testCompany",
            "label": "TST",
            "description": "company for tests",
            "deliveriesFrom": ["LBR"],
            "logo": "TST",
            "currency": "STQ"
        }
    }
}
''',

'updateCompany' : '''
{"query":
    "mutation updateCompany($input: UpdateCompanyInput!) {updateCompany (input: $input) {deliveriesFrom {alpha3 children{alpha3 children{alpha3}}}}}",
    "variables": {
         "input": {
            "clientMutationId": "",
			"id": "%(company_id)s",
            "deliveriesFrom": ["LBR", "RUS"],
            "logo": "XxX_TST_XxX"
        }
    }
}
''',

'createPackage' : '''
{"query":
    "mutation createPackage($input: NewPackagesInput!) {createPackage (input: $input) {id, rawId}}",
    "variables": {
        "input": {
            "clientMutationId": "",
            "name": "testPackge",
            "maxSize": 200,
            "minSize": 5,
            "maxWeight": 30000,
            "minWeight": 100,
            "deliveriesTo": ["LBR"]
        }
    }
}
''',

'updatePackage' : '''
{"query":
    "mutation updatePackage($input: UpdatePackagesInput!) {updatePackage (input: $input) {deliveriesTo {alpha3 children{alpha3 children{alpha3}}}}}",
    "variables": {
        "input": {
            "clientMutationId": "",
            "id": "%(package_id)s",
            "maxSize": 210,
            "minSize": 10,
            "maxWeight": 33000,
            "minWeight": 100,
            "deliveriesTo": ["LBR", "RUS"]
        }
    }
}
''',

'addPackageToCompany' : '''
{"query":
    "mutation addPackageToCompany($input: NewCompaniesPackagesInput!) {addPackageToCompany (input: $input) {id, rawId}}",
    "variables": {
         "input": {
            "clientMutationId": "",
			"companyId": %(company_rawid)i,
          	"packageId": %(package_rawid)i 
        }
    }
}
''',

'createUser' : '''
{"query":
	"mutation createUser($input: CreateUserInput!) {createUser(input: $input) {id rawId}}",
	"variables": {
	    "input": {
	        "clientMutationId": "",
	        "firstName": "tester",
	        "lastName": "testoviy",
	        "email": "%(regmail)s",
	        "password": "qwe123QWE" 
	    }
	},
	"operationName": "createUser"
}
''',

'addRoleToUserOnUsersMicroservice' : '''
{"query":
    "mutation addRoleToUserOnUsersMicroservice($input: NewUsersRoleInput!) {addRoleToUserOnUsersMicroservice (input: $input) {userId, name}}",
    "variables": {
        "input": {
            "clientMutationId": "",
            "userId": %(new_usr_rawid)i,
            "name": "MODERATOR"
        }
    }
}
''',

'addRoleToUserOnStoresMicroservice' : '''
{"query":
    "mutation addRoleToUserOnStoresMicroservice($input: NewStoresRoleInput!) {addRoleToUserOnStoresMicroservice (input: $input) {userId, name}}",
    "variables": {
        "input": {
            "clientMutationId": "",
            "userId": %(new_usr_rawid)i,
            "name": "MODERATOR"
        }
    }
}
''',

'removeRoleFromUserOnUsersMicroservice' : '''
{"query":
    "mutation removeRoleFromUserOnUsersMicroservice($input: RemoveUsersRoleInput!) {removeRoleFromUserOnUsersMicroservice (input: $input) {userId, name}}",
    "variables": {
        "input": {
            "clientMutationId": "",
            "userId": %(new_usr_rawid)i,
            "name": "MODERATOR"
        }
    }
}
''',

'removeRoleFromUserOnStoresMicroservice' : '''
{"query":
    "mutation removeRoleFromUserOnStoresMicroservice($input: RemoveStoresRoleInput!) {removeRoleFromUserOnStoresMicroservice (input: $input) {userId, name}}",
    "variables": {
        "input": {
            "clientMutationId": "",
            "userId": %(new_usr_rawid)i,
            "name": "MODERATOR"
        }
    }
}
''',

'user_getJWTByEmail' : '''
{"query":
	"mutation getJWTByEmail($input: CreateJWTEmailInput!) {getJWTByEmail (input: $input) {token}}",
	"variables": {
	    "input": {
	        "clientMutationId": "",
	        "email": "%(usr)s",
	        "password": "%(usrpwd)s"
	    }
	}
}
''',

'query_me' : '''
{"query":
	"query {me {id, rawId, isActive, myStore {id}}}" }
''',

'updateUser' : '''
{"query":
    "mutation updateUser($input: UpdateUserInput!) {updateUser(input: $input){id, isActive}}",
    "variables": {
        "input": {
            "clientMutationId": "",
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
            "clientMutationId": "",
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
            "clientMutationId": "",
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

'createWizardStore' : '''
{"query":
    "mutation createWizardStore {createWizardStore {id, storeId}}"
}
''',

'updateWizardStore' : '''
{"query":
    "mutation updateWizardStore($input: UpdateWizardStoreInput!) {updateWizardStore(input: $input) {id, storeId}}",    
    "variables": {
        "input" : {
            "clientMutationId": "",
          	"addressFull": {"country": "Russia", "postalCode": "123321"}
        }
    }
}
''',

'deleteWizardStore' : '''
{"query":
    "mutation deleteWizardStore {deleteWizardStore {id, storeId}}"
}
''',

'createStore' : '''
{"query":
    "mutation createStore($input: CreateStoreInput!) {createStore(input: $input) {id, name{lang, text}, rawId}}",
    "variables": {
        "input": {
            "clientMutationId": "",
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

'updateStore' : '''
{"query":
    "mutation updateStore($input: UpdateStoreInput!) {updateStore(input: $input) {id, isActive}}",
    "variables": {
        "input": {
            "clientMutationId": "",
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

'createWarehouse' : '''
{"query":
    "mutation createWarehouse($input: CreateWarehouseInput!) {createWarehouse (input: $input) {id}}",
    "variables": {
        "input": {
            "clientMutationId": "",
            "name": "testwar",
            "slug": "testwar",
            "storeId": %(store_rawid)i,
            "addressFull": {"value": "gdeto", "country": "Canada", "postalCode": "111111", "countryCode": "CND"}
        }
    }
}
''',

'updateWarehouse' : '''
{"query":
    "mutation updateWarehouse($input: UpdateWarehouseInput!) {updateWarehouse (input: $input) {id, name}}",
    "variables": {
        "input": {
            "clientMutationId": "",
            "name": "testwarE",
            "slug": "testwar%(n)s",
            "id": "%(war_id)s",
            "addressFull": {"value": "gdeto", "country": "Liberia", "postalCode": "111111", "countryCode": "LBR"}
        }
    }
}
''',

'createBaseProduct' : '''
{"query":
    "mutation createBaseProduct($input: CreateBaseProductInput!) {createBaseProduct(input: $input) {id rawId name {lang text}}}",
    "variables": {
        "input" : {
            "clientMutationId": "",
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

'updateBaseProduct' : '''
{"query":
    "mutation updateBaseProduct($input: UpdateBaseProductInput!) {updateBaseProduct(input: $input) {id, rawId}}",
    "variables": {
        "input" : {
            "clientMutationId": "",
             "id": "%(b_prod_id)s",
             "longDescription" : [{"lang" : "EN", "text" : "Long Desc"}]
        }
    }
}
''',

'createCustomAttribute' : '''
{"query":
    "mutation createCustomAttribute($input: NewCustomAttributeInput!) {createCustomAttribute (input: $input) {id, rawId}}",
    "variables": {
        "input":{
            "clientMutationId": "",
			"attributeId": %(attr_rawid)i,
            "baseProductId": %(b_prod_rawid)i
        }
    }
}
''',

'createProduct' : '''
{"query":
    "mutation createProduct($input: CreateProductWithAttributesInput!) {createProduct(input: $input) {id, isActive, rawId}}",
    "variables": {
        "input" : {
            "clientMutationId": "",
             "product":  {"baseProductId": %(b_prod_rawid)i,
                          "preOrder": true,
                          "preOrderDays": 10,
                          "price": 200.00,
                          "vendorCode": "11"},
             "attributes": [{"attrId": %(attr_rawid)i,
                             "value": "test1",
                             "metaField": "dfasfas"}]   
        }
    }
}
''',

'updateProduct' : '''
{"query":
    "mutation updateProduct($input: UpdateProductWithAttributesInput!) {updateProduct(input: $input) {id}}",
    "variables": {
        "input": {
            "clientMutationId": "",
            "id": "%(prod_id)s",
            "product": {"discount": 1.0}
        }
    }
}
''',

'upsertShipping' : '''
{"query":
    "mutation upsertShipping($input: NewShippingInput!) {upsertShipping (input: $input) {local {deliveriesTo {alpha3 children{alpha3 children{alpha3}}}}}}",
    "variables": {
        "input": {
            "clientMutationId": "",
            "local": {"companyPackageId": %(comp_pack_rawid)i, "price": 100.0},
            "international": {"companyPackageId": %(comp_pack_rawid)i, "price": 200.0, "deliveriesTo": ["RUS"]},
            "pickup": {"pickup": true, "price": 0.0},
            "baseProductId": %(b_prod_rawid)i,
			"storeId": %(store_rawid)i
        }
    }
}
''',

'createBaseProductWithVariants' : '''
{"query":
    "mutation createBaseProductWithVariants($input: NewBaseProductWithVariantsInput!) {createBaseProductWithVariants(input: $input) {id, rawId, products(first : 100) {edges {node {id, rawId}}}}}",
    "variables": {
        "input" : {
            "clientMutationId": "",
            "name": [{"lang": "EN", "text": "testprodwithvar"}],
          	"storeId": %(store_rawid)i,
          	"shortDescription": [{"lang": "EN", "text": "test"}],
          	"currency": "STQ",
          	"categoryId": %(cat_rawid_3)i,
          	"variants": {
              "clientMutationId": "",
              "product": {
                "vendorCode": "E1",
                "price": 10.0
              },
            	"attributes": [{"attrId": %(attr_rawid)i, "value": "test2"}]
            },
          	"selectedAttributes": [%(attr_rawid)i]
        }     
    }
}
''',

'createCoupon' : '''
{"query":
    "mutation createCoupon($input: NewCouponInput!) {createCoupon(input: $input) {id rawId code isActive}}",
"variables" : {
    "input": {
          "clientMutationId": "",
          "code": "TEST1",
          "title": "skidosik",
          "storeId": %(store_rawid)i,
          "scope": "BASE_PRODUCTS",
          "percent": 10,
          "quantity": 10
    }
 }
}
''',

'setProductQuantityInWarehouse' : '''
{"query":
    "mutation setProductQuantityInWarehouse($input: ProductQuantityInput!) {setProductQuantityInWarehouse (input: $input) {id, productId, quantity}}",
    "variables": {
        "input":{
            "clientMutationId": "",
            "warehouseId": "%(war_id)s",
            "productId": %(prod_rawid)i,
            "quantity": 33
        }
    }
}
''',

'sendStoreToModeration' : '''
{"query" :
    "mutation sendStoreToModeration {sendStoreToModeration(id:%(store_rawid)i) {rawId status}}"
}
''',

'setModerationStatusStore' : '''
{"query" :
    "mutation setModerationStatusStore($input: StoreModerateInput!) {setModerationStatusStore(input: $input) {rawId status}}",
"variables": {
  "input": {
	"id" : "%(store_id)s",
    "status" : "PUBLISHED"
  }
}
}
''',

'sendBaseProductToModeration' : '''
{"query" :
    "mutation sendBaseProductToModeration {sendBaseProductToModeration(id:%(b_prod_rawid)i) {rawId status}}"
}
''',

'setModerationStatusBaseProduct' : '''
{"query" :
    "mutation setModerationStatusBaseProduct($input: BaseProductModerateInput!) {setModerationStatusBaseProduct(input: $input) {rawId status}}",
"variables": {
  "input": {
	"id" : "%(b_prod_id)s",
    "status" : "PUBLISHED"
  }
}
}
''',

'clearCart' : '''
{"query":
    "mutation clearCart  {clearCart{id, totalCost}}"}
''',

'incrementInCart' : '''
{"query":
    "mutation incrementInCart($input: IncrementInCartInput!) {incrementInCart(input: $input) {id, productsCost}}",
"variables": {
    "input": {
        "clientMutationId": "",
        "productId": %(prod_rawid)i
    }
 }
}
''',

'setSelectionInCart' : '''
{"query":
    "mutation setSelectionInCart($input: SetSelectionInCartInput!) {setSelectionInCart(input: $input) {id, productsCost}}",
"variables": {
    "input": {
        "clientMutationId": "",
        "productId": %(prod_rawid)i,
        "value": true
    }
 }
}
''',

'setQuantityInCart' : '''
{"query":
    "mutation setQuantityInCart($input: SetQuantityInCartInput!) {setQuantityInCart(input: $input) {id, productsCost}}",
"variables": {
    "input": {
        "clientMutationId": "",
        "productId": %(prod_rawid)i,
        "value": 3
    }
 }
}
''',

'availableShippingForUser' : '''
{"query":
    "{availableShippingForUser(userCountry: \\"RUS\\", baseProductId: %(b_prod_rawid)i) {packages {id shippingId}}}"
}
''',

'setDeliveryMethodInCart' : '''
{"query":
    "mutation setDeliveryMethodInCart($input: SetDeliveryMethodInCartInput!) {setDeliveryMethodInCart(input: $input) {id productsCost totalCost}}",
"variables": {
    "input": {
            "clientMutationId": "",
			"productId": %(prod_rawid)i,
          	"shippingId": %(shipping_id)i
    }
 }
}
''',

'createOrders' : '''
{"query":
    "mutation createOrders($input: CreateOrderInput!) {createOrders (input: $input) {invoice{id, orders{id, slug, trackId}}}}",
"variables": {
    "input": {
        "clientMutationId": "",
        "addressFull": {"value": "gdeto", "country": "Russian Federation", "postalCode": "111111", "countryCode": "RUS"},
        "receiverName": "tester",
        "receiverPhone": "+79095623366",
        "currency": "STQ" 
    }
 }
}
''',

'setOrderStatusDelivery' : '''
{"query":
    "mutation setOrderStatusDelivery($input: OrderStatusDeliveryInput!) {setOrderStatusDelivery (input: $input) {state, trackId}}",
"variables": {
    "input": {
        "clientMutationId": "",
        "orderSlug": %(order_slug)i,
        "comment": "test"
    }
 }
}
''',

'order_history' : '''
{"query":
    "query order_history {me{id, order(slug: %(order_slug)i){history {edges {node {state, orderId, comment}}}}}}"}
''',

'setOrderStatusCanceled' : '''
{"query":
    "mutation setOrderStatusCanceled($input: OrderStatusCanceledInput!) {setOrderStatusCanceled (input: $input) {state, trackId}}",
"variables": {
    "input": {
        "clientMutationId": "",
        "orderSlug": %(order_slug)i,
        "comment": "test"
    }
 }}
''',

'setOrderStatusComplete' : '''
{"query":
    "mutation setOrderStatusComplete($input: OrderStatusCompleteInput!) {setOrderStatusComplete (input: $input) {state, trackId}}",
"variables": {
    "input": {
        "clientMutationId": "",
        "orderSlug": %(order_slug)i,
        "comment": "test"
    }
 }}
''',

'deleteFromCart' : '''
{"query":
    "mutation deleteFromCart($input: DeleteFromCartInput!) {deleteFromCart(input: $input) {id, totalCost}}",
"variables": {
    "input": {
        "clientMutationId": "",
        "productId": %(prod_rawid)i
    }
 }}
''',

'buyNow' : '''
{"query":
    "mutation buyNow($input: BuyNowInput!) {buyNow(input: $input) {invoice {id state} cart{id totalCost}}}",
"variables": {
    "input": {
        "clientMutationId": "",
        "productId": %(prod_rawid)i,
        "quantity": 1,
        "addressFull": {"value": "gdeto", "country": "Russian Federation", "postalCode": "111111", "countryCode": "RUS"},
        "receiverName": "apitester",
        "receiverPhone": "+79034569874",
        "currency": "RUB",
        "shippingId": %(shipping_id)i
    }
 }
}
''',

'recalcInvoiceAmount' : '''
{"query":
    "mutation recalcInvoiceAmount{recalcInvoiceAmount(id : \\"%(invoice_id)s\\") {id} }"
}
''',

'draftBaseProducts' : '''
{"query":
    "mutation draftBaseProducts {draftBaseProducts(ids: %(b_prod_rawid)i) {id}}"
}
''',

'draftStore': '''
{"query":
    "mutation draftStore {draftStore(id: %(store_rawid)i) {id}}"
}
''',

'deleteCompanyPackage' : '''
{"query":
    "mutation deleteCompanyPackage {deleteCompanyPackage(companyId: %(company_rawid)i, packageId: %(package_rawid)i) {id}}"
}
''',

'deletePackage' : '''
{"query":
    "mutation deletePackage {deletePackage(id: %(package_rawid)i) {id}}"
}
''',

'deleteCompany' : '''
{"query":
    "mutation deleteCompany {deleteCompany(id: %(company_rawid)i) {id}}"
}
''',

'deleteAttributeFromCategory' : '''
{"query":
    "mutation deleteAttributeFromCategory($input: DeleteAttributeFromCategory!) {deleteAttributeFromCategory(input: $input) {mock}}",
    "variables": {
        "input": {
            "clientMutationId": "",
            "catId": %(cat_rawid_3)i,
            "attrId": %(attr_rawid)i
        }
    }
}
''',

'deactivateProduct' : '''
{"query":
    "mutation deactivateProduct($input: DeactivateProductInput!) {deactivateProduct(input: $input) {id, isActive}}",
    "variables": {
        "input": {
            "clientMutationId": "",
            "id": "%(prod_id)s"
        }
    }
}
''',

'deactivateBaseProduct' : '''
{"query":
    "mutation deactivateBaseProduct($input: DeactivateBaseProductInput!) {deactivateBaseProduct(input: $input) {id, isActive}}",
    "variables": {
        "input": {
            "clientMutationId": "",
            "id": "%(b_prod_id)s"
        }
    }
}
''',

'deactivateStore' : '''
{"query":
    "mutation deactivateStore($input: DeactivateStoreInput!) {deactivateStore(input: $input) {id, isActive}}",
    "variables": {
        "input":{
            "clientMutationId": "",
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
#
# 'get_b_prods' : '''
# {"query":
#     "{me {baseProducts {edges {node {id rawId}}}}}"
# }
# ''',
#
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
# 'publish_store' : '''
# {"query":
#     "mutation publishStore {publishStore(id: %(store_rawid)i) {id}}"
# }
# ''',
#
# 'publish_b_prod' : '''
# {"query":
#     "mutation publishBaseProducts {publishBaseProducts(ids: %(b_prod_rawid)i) {id}}"
# }
# ''',