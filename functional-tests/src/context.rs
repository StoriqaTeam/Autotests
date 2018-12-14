use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;

use config::Config;
use microservice::*;
use query::*;
use request::GraphqlRequest;

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
            .request(get_jwt_by_email::CreateJWTEmailInput {
                client_mutation_id: "".to_string(),
                email: "admin@storiqa.com".to_string(),
                password: "bqF5BkdsCS".to_string(),
            })
            .expect("get_jwt_by_email failed")
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

    pub fn users_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.users_microservice.healthcheck()?;
        Ok(())
    }

    pub fn stores_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.stores_microservice.healthcheck()?;
        Ok(())
    }

    pub fn orders_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.orders_microservice.healthcheck()?;
        Ok(())
    }

    pub fn warehouses_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.warehouses_microservice.healthcheck()?;
        Ok(())
    }

    pub fn billing_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.billing_microservice.healthcheck()?;
        Ok(())
    }

    pub fn notifications_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.notifications_microservice.healthcheck()?;
        Ok(())
    }

    pub fn delivery_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.delivery_microservice.healthcheck()?;
        Ok(())
    }

    pub fn saga_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.saga_microservice.healthcheck()?;
        Ok(())
    }

    pub fn gateway_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.gateway_microservice.healthcheck()?;
        Ok(())
    }

    pub fn request<T: GraphqlRequest>(&self, input: T) -> Result<T::Output, FailureError> {
        let payload: serde_json::Value = input.into();
        let response_body: serde_json::Value = self.graphql_request(payload)?;
        T::response(response_body)
    }

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

    pub fn delete_delivery_company(
        &self,
        id: i64,
    ) -> Result<delete_delivery_company::ResponseData, FailureError> {
        let request_body = delete_delivery_company::DeleteCompanyMutation::build_query(
            delete_delivery_company::Variables { id },
        );
        let response_body: Response<delete_delivery_company::ResponseData> =
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
