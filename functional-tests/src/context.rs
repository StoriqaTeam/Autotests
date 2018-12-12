use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;

use config::Config;
use microservice::*;
use query::*;

pub struct TestContext {
    bearer: Option<String>,
    client: Client,
    config: Config,
    users_microservice: UsersMicroservice,
    stores_microservice: StoresMicroservice,
    orders_microservice: OrdersMicroservice,
    warehouses_microservice: WarehousesMicroservice,
    billing_microservice: BillingMicroservice,
    notifications_microservice: NotificationsMicroservice,
    delivery_microservice: DeliveryMicroservice,
    saga_microservice: SagaMicroservice,
    gateway_microservice: GatewayMicroservice,
}

macro_rules! graphql_request {
    ($func_name:ident, $mod_name:ident, $input:ident, $struct_name:ident) => {
        pub fn $func_name(&self, input: $mod_name::$input) -> Result<$mod_name::ResponseData, FailureError> {
            let request_body = $mod_name::$struct_name::build_query(
                $mod_name::Variables { input },
            );
            let response_body: Response<$mod_name::ResponseData> =
                self.graphql_request(request_body)?;
            match (response_body.data, response_body.errors) {
                (Some(data), None) => Ok(data),
                (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
                _ => unreachable!()
            }
        }
    }
}

impl TestContext {
    pub fn new() -> TestContext {
        let client = Client::new();

        let config = Config::new().expect("Could not read config");

        let context = TestContext {
            users_microservice: UsersMicroservice {
                url: config.users_microservice.url.clone(),
                database_url: config.users_microservice.database_url.clone(),
                client: client.clone(),
            },
            stores_microservice: StoresMicroservice {
                url: config.stores_microservice.url.clone(),
                database_url: config.stores_microservice.database_url.clone(),
                client: client.clone(),
            },
            orders_microservice: OrdersMicroservice {
                url: config.orders_microservice.url.clone(),
                database_url: config.orders_microservice.database_url.clone(),
                client: client.clone(),
            },
            warehouses_microservice: WarehousesMicroservice {
                url: config.warehouses_microservice.url.clone(),
                database_url: config.warehouses_microservice.database_url.clone(),
                client: client.clone(),
            },
            billing_microservice: BillingMicroservice {
                url: config.billing_microservice.url.clone(),
                database_url: config.billing_microservice.database_url.clone(),
                client: client.clone(),
            },
            notifications_microservice: NotificationsMicroservice {
                url: config.notifications_microservice.url.clone(),
                database_url: config.notifications_microservice.database_url.clone(),
                client: client.clone(),
            },
            delivery_microservice: DeliveryMicroservice {
                url: config.delivery_microservice.url.clone(),
                database_url: config.delivery_microservice.database_url.clone(),
                client: client.clone(),
            },
            saga_microservice: SagaMicroservice {
                url: config.saga_microservice.url.clone(),
                database_url: config.saga_microservice.database_url.clone(),
                client: client.clone(),
            },
            gateway_microservice: GatewayMicroservice {
                url: config.gateway_microservice.url.clone(),
                client: client.clone(),
            },
            config,
            bearer: None,
            client: client.clone(),
        };

        context.clear_all_data().unwrap();
        context
    }

    pub fn verify_user_email(&self, email: &str) -> Result<(), FailureError> {
        self.users_microservice.verify_email(email)
    }

    pub fn get_email_verification_token(&self, email: &str) -> Result<String, FailureError> {
        self.users_microservice.get_email_verification_token(email)
    }

    pub fn clear_all_data(&self) -> Result<(), FailureError> {
        self.users_microservice.clear_all_data()?;
        self.stores_microservice.clear_all_data()?;
        Ok(())
    }

    pub fn set_bearer(&mut self, bearer: String) {
        self.bearer = Some(bearer);
    }

    pub fn as_superadmin(&mut self) {
        let token: String = self
            .get_jwt_by_email(get_jwt_by_email::CreateJWTEmailInput {
                client_mutation_id: "".to_string(),
                email: "admin@storiqa.com".to_string(),
                password: "bqF5BkdsCS".to_string(),
            })
            .unwrap()
            .get_jwt_by_email
            .token;
        self.bearer = Some(token);
    }

    pub fn clear_bearer(&mut self) {
        self.bearer = None;
    }

    pub fn get_attributes(&self) -> Result<get_attributes::ResponseData, FailureError> {
        let request_body =
            get_attributes::GetAttributesQuery::build_query(get_attributes::Variables {});
        let response_body: Response<get_attributes::ResponseData> =
            self.graphql_request(request_body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }

    pub fn get_categories(&self) -> Result<get_categories::ResponseData, FailureError> {
        let request_body =
            get_categories::GetCategoriesQuery::build_query(get_categories::Variables {});
        let response_body: Response<get_categories::ResponseData> =
            self.graphql_request(request_body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }

    pub fn get_store(&self, store_id: i64) -> Result<get_store::ResponseData, FailureError> {
        let request_body = get_store::GetStoreQuery::build_query(get_store::Variables {
            id: store_id,
            visibility: Some(get_store::Visibility::Active),
        });
        let response_body: Response<get_store::ResponseData> =
            self.graphql_request(request_body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }

    pub fn get_base_product(
        &self,
        base_product_id: i64,
    ) -> Result<get_base_product::ResponseData, FailureError> {
        let request_body =
            get_base_product::GetBaseProductQuery::build_query(get_base_product::Variables {
                id: base_product_id,
                visibility: Some(get_base_product::Visibility::Active),
            });
        let response_body: Response<get_base_product::ResponseData> =
            self.graphql_request(request_body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }

    pub fn microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.users_microservice.healthcheck()?;
        self.stores_microservice.healthcheck()?;
        self.orders_microservice.healthcheck()?;
        self.warehouses_microservice.healthcheck()?;
        self.billing_microservice.healthcheck()?;
        self.notifications_microservice.healthcheck()?;
        self.delivery_microservice.healthcheck()?;
        self.saga_microservice.healthcheck()?;
        self.gateway_microservice.healthcheck()?;
        Ok(())
    }

    graphql_request!(
        create_category,
        create_category,
        CreateCategoryInput,
        CreateCategoryMutation
    );

    graphql_request!(
        delete_attribute_from_category,
        delete_attribute_from_category,
        DeleteAttributeFromCategory,
        DeleteAttributeFromCategoryMutation
    );

    graphql_request!(
        add_attribute_to_category,
        add_attribute_to_category,
        AddAttributeToCategoryInput,
        AddAttributeToCategoryMutation
    );

    graphql_request!(
        update_category,
        update_category,
        UpdateCategoryInput,
        UpdateCategoryMutation
    );

    graphql_request!(
        delete_category,
        delete_category,
        DeleteCategoryInput,
        DeleteCategoryMutation
    );

    graphql_request!(
        create_user,
        create_user,
        CreateUserInput,
        CreateUserMutation
    );

    graphql_request!(
        update_user,
        update_user,
        UpdateUserInput,
        UpdateUserMutation
    );

    graphql_request!(
        deactivate_user,
        deactivate_user,
        DeactivateUserInput,
        DeactivateUserMutation
    );

    pub fn delete_user(&self, user_id: i64) -> Result<delete_user::ResponseData, FailureError> {
        let request_body =
            delete_user::DeleteUserMutation::build_query(delete_user::Variables { user_id });
        let response_body: Response<delete_user::ResponseData> =
            self.graphql_request(request_body)?;

        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }

    graphql_request!(
        create_user_jwt,
        get_jwt_by_provider,
        CreateJWTProviderInput,
        GetJwtByProviderMutation
    );
    graphql_request!(
        get_jwt_by_email,
        get_jwt_by_email,
        CreateJWTEmailInput,
        GetJwtByEmailMutation
    );
    graphql_request!(
        verify_email,
        verify_email,
        VerifyEmailApply,
        VerifyEmailMutation
    );
    graphql_request!(
        create_store,
        create_store,
        CreateStoreInput,
        CreateStoreMutation
    );
    graphql_request!(
        update_store,
        update_store,
        UpdateStoreInput,
        UpdateStoreMutation
    );
    graphql_request!(
        create_attribute,
        create_attribute,
        CreateAttributeInput,
        CreateAttributeMutation
    );
    graphql_request!(
        update_attribute,
        update_attribute,
        UpdateAttributeInput,
        UpdateAttributeMutation
    );
    graphql_request!(
        delete_attribute,
        delete_attribute,
        DeleteAttributeInput,
        DeleteAttributeMutation
    );
    graphql_request!(
        create_attribute_value,
        create_attribute_value,
        CreateAttributeValueInput,
        CreateAttributeValueMutation
    );
    graphql_request!(
        update_attribute_value,
        update_attribute_value,
        UpdateAttributeValueInput,
        UpdateAttributeValueMutation
    );
    graphql_request!(
        delete_attribute_value,
        delete_attribute_value,
        DeleteAttributeValueInput,
        DeleteAttributeValueMutation
    );

    graphql_request!(
        create_base_product,
        create_base_product,
        CreateBaseProductInput,
        CreateBaseProductMutation
    );

    graphql_request!(
        deactivate_base_product,
        deactivate_base_product,
        DeactivateBaseProductInput,
        DeactivateBaseProductMutation
    );

    graphql_request!(
        create_base_product_with_variants,
        create_base_product_with_variants,
        NewBaseProductWithVariantsInput,
        CreateBaseProductWithVariantsMutation
    );

    graphql_request!(
        update_base_product,
        update_base_product,
        UpdateBaseProductInput,
        UpdateBaseProductMutation
    );

    graphql_request!(
        update_product,
        update_product,
        UpdateProductWithAttributesInput,
        UpdateProductMutation
    );

    graphql_request!(
        create_product,
        create_product,
        CreateProductWithAttributesInput,
        CreateProductMutation
    );

    graphql_request!(
        deactivate_product,
        deactivate_product,
        DeactivateProductInput,
        DeactivateProductMutation
    );

    pub fn delete_store(&self, store_id: i64) -> Result<delete_store::ResponseData, FailureError> {
        let request_body =
            delete_store::DeleteStoreMutation::build_query(delete_store::Variables { store_id });
        let response_body: Response<delete_store::ResponseData> =
            self.graphql_request(request_body)?;

        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }

    fn graphql_request<T: Serialize, S: DeserializeOwned>(
        &self,
        data: T,
    ) -> Result<S, FailureError> {
        let mut request = self
            .client
            .post(&self.config.gateway_microservice.graphql_url)
            .header("Currency", "STQ");
        if let Some(ref bearer) = self.bearer {
            request = request.header("Authorization", format!("Bearer {}", bearer));
        }
        let mut res = request.json(&data).send()?;
        let result = res.json()?;
        Ok(result)
    }
}
