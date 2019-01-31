use std::sync::Arc;

use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;

use config::{Config, RunMode};
use microservice::*;
use query::*;
use request::GraphqlRequest;

trait DataContext {
    fn verify_user_email(&self, email: &str) -> Result<(), FailureError>;
    fn graphql_url(&self) -> String;
    fn clear_all_data(&self) -> Result<(), FailureError>;
    fn microservice_healthcheck(&self) -> Result<(), FailureError>;
    fn users_microservice_healthcheck(&self) -> Result<(), FailureError>;
    fn stores_microservice_healthcheck(&self) -> Result<(), FailureError>;
    fn orders_microservice_healthcheck(&self) -> Result<(), FailureError>;
    fn warehouses_microservice_healthcheck(&self) -> Result<(), FailureError>;
    fn billing_microservice_healthcheck(&self) -> Result<(), FailureError>;
    fn notifications_microservice_healthcheck(&self) -> Result<(), FailureError>;
    fn delivery_microservice_healthcheck(&self) -> Result<(), FailureError>;
    fn saga_microservice_healthcheck(&self) -> Result<(), FailureError>;
    fn gateway_microservice_healthcheck(&self) -> Result<(), FailureError>;
}

#[derive(Clone)]
pub struct MicroserviceDataContextImpl {
    pub graphql_url: String,
    pub users_microservice: UsersMicroservice,
    pub stores_microservice: StoresMicroservice,
    pub orders_microservice: OrdersMicroservice,
    pub warehouses_microservice: WarehousesMicroservice,
    pub billing_microservice: BillingMicroservice,
    pub notifications_microservice: NotificationsMicroservice,
    pub delivery_microservice: DeliveryMicroservice,
    pub saga_microservice: SagaMicroservice,
    pub gateway_microservice: GatewayMicroservice,
}

impl MicroserviceDataContextImpl {
    pub fn new(config: Config, client: Client, graphql_url: String) -> Self {
        Self {
            graphql_url,
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
                client: client.clone(),
            },
            gateway_microservice: GatewayMicroservice {
                url: config.gateway_microservice.url.clone(),
                client: client.clone(),
            },
        }
    }
}

impl DataContext for MicroserviceDataContextImpl {
    fn graphql_url(&self) -> String {
        self.graphql_url.to_string()
    }

    fn verify_user_email(&self, email: &str) -> Result<(), FailureError> {
        self.users_microservice.verify_email(email)?;
        Ok(())
    }

    fn clear_all_data(&self) -> Result<(), FailureError> {
        self.users_microservice.clear_all_data()?;
        self.stores_microservice.clear_all_data()?;
        self.orders_microservice.clear_all_data()?;
        self.notifications_microservice.clear_all_data()?;
        self.delivery_microservice.clear_all_data()?;
        self.billing_microservice.clear_all_data()?;
        self.warehouses_microservice.clear_all_data()?;

        Ok(())
    }

    fn microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.users_microservice.healthcheck()?;
        self.stores_microservice.healthcheck()?;
        self.orders_microservice.healthcheck()?;
        self.notifications_microservice.healthcheck()?;
        self.delivery_microservice.healthcheck()?;
        self.billing_microservice.healthcheck()?;
        self.warehouses_microservice.healthcheck()?;
        Ok(())
    }

    fn users_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.users_microservice.healthcheck()?;
        Ok(())
    }

    fn stores_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.stores_microservice.healthcheck()?;
        Ok(())
    }

    fn orders_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.orders_microservice.healthcheck()?;
        Ok(())
    }

    fn warehouses_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.warehouses_microservice.healthcheck()?;
        Ok(())
    }

    fn billing_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.billing_microservice.healthcheck()?;
        Ok(())
    }
    fn notifications_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.notifications_microservice.healthcheck()?;
        Ok(())
    }
    fn delivery_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.delivery_microservice.healthcheck()?;
        Ok(())
    }
    fn saga_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.saga_microservice.healthcheck()?;
        Ok(())
    }

    fn gateway_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.gateway_microservice.healthcheck()?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct HttpDataContextImpl {
    pub test_tools_url: String,
    pub graphql_url: String,
    pub client: Client,
}

impl HttpDataContextImpl {
    pub fn new(client: Client, graphql_url: String, test_tools_url: String) -> Self {
        Self {
            client,
            graphql_url,
            test_tools_url,
        }
    }

    pub fn build_request(&self, request_path: &str) -> reqwest::RequestBuilder {
        let path = format!("{}/{}", self.test_tools_url, request_path);
        self.client.post(&path).header("cookie", "holyshit=iamcool")
    }

    pub fn send_request(&self, request_path: &str) -> reqwest::Result<reqwest::Response> {
        self.build_request(request_path).send()
    }
}

#[derive(Deserialize, Serialize)]
pub struct VerifyUserEmail {
    email: String,
}

impl DataContext for HttpDataContextImpl {
    fn graphql_url(&self) -> String {
        self.graphql_url.to_string()
    }

    fn verify_user_email(&self, email: &str) -> Result<(), FailureError> {
        let payload = VerifyUserEmail {
            email: email.to_string(),
        };

        self.build_request("verify_user_email")
            .json(&payload)
            .send()?;

        Ok(())
    }

    fn clear_all_data(&self) -> Result<(), FailureError> {
        self.send_request("clear_all_data")?;
        Ok(())
    }

    fn microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.send_request("microservice_healthcheck")?;
        Ok(())
    }

    fn users_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.send_request("users_microservice_healthcheck")?;
        Ok(())
    }

    fn stores_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.send_request("stores_microservice_healthcheck")?;
        Ok(())
    }

    fn orders_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.send_request("orders_microservice_healthcheck")?;
        Ok(())
    }

    fn warehouses_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.send_request("warehouses_microservice_healthcheck")?;
        Ok(())
    }

    fn billing_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.send_request("billing_microservice_healthcheck")?;
        Ok(())
    }
    fn notifications_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.send_request("notifications_microservice_healthcheck")?;
        Ok(())
    }
    fn delivery_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.send_request("delivery_microservice_healthcheck")?;
        Ok(())
    }
    fn saga_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.send_request("saga_microservice_healthcheck")?;
        Ok(())
    }

    fn gateway_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.send_request("gateway_microservice_healthcheck")?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct TestContext {
    bearer: Option<String>,
    currency: String,
    fiat_currency: String,
    client: Client,
    config: Config,
    data_context: Arc<dyn DataContext + 'static + Send + Sync>,
}

impl TestContext {
    pub fn with_config(config: Config) -> TestContext {
        let client = Client::new();

        let data_context = match config
            .test_environment
            .clone()
            .expect("Cannot get test environment")
        {
            RunMode::Local { graphql_url } => Arc::new(MicroserviceDataContextImpl::new(
                config.clone(),
                client.clone(),
                graphql_url,
            ))
                as Arc<dyn DataContext + 'static + Send + Sync>,
            RunMode::Cluster {
                graphql_url,
                test_tools_url,
            } => Arc::new(HttpDataContextImpl::new(
                client.clone(),
                graphql_url,
                test_tools_url,
            )) as Arc<dyn DataContext + 'static + Send + Sync>,
        };

        Self {
            config,
            bearer: None,
            currency: "STQ".to_string(),
            fiat_currency: "EUR".to_string(),
            client: client.clone(),
            data_context,
        }
    }

    pub fn new() -> TestContext {
        let config = Config::new().expect("Could not read config");
        let context = TestContext::with_config(config);

        context.clear_all_data().expect("Cannot clear data");
        context
    }

    pub fn set_currency(&mut self, currency: impl Into<String>) {
        self.currency = currency.into();
    }

    pub fn set_fiat_currency(&mut self, fiat_currency: impl Into<String>) {
        self.fiat_currency = fiat_currency.into();
    }

    pub fn verify_user_email(&self, email: &str) -> Result<(), FailureError> {
        self.data_context.verify_user_email(email)
    }

    pub fn clear_all_data(&self) -> Result<(), FailureError> {
        self.data_context.clear_all_data()?;

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

    pub fn get_categories_with_products(
        &self,
    ) -> Result<get_categories_with_products::ResponseData, FailureError> {
        let request_body =
            get_categories_with_products::GetCategoriesWithProductsQuery::build_query(
                get_categories_with_products::Variables {},
            );
        let response_body: Response<get_categories_with_products::ResponseData> =
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
        self.data_context.users_microservice_healthcheck()?;
        self.data_context.stores_microservice_healthcheck()?;
        self.data_context.orders_microservice_healthcheck()?;
        self.data_context.warehouses_microservice_healthcheck()?;
        self.data_context.billing_microservice_healthcheck()?;
        self.data_context.notifications_microservice_healthcheck()?;
        self.data_context.delivery_microservice_healthcheck()?;
        self.data_context.saga_microservice_healthcheck()?;
        self.data_context.gateway_microservice_healthcheck()?;

        Ok(())
    }

    pub fn users_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.data_context.users_microservice_healthcheck()?;
        Ok(())
    }

    pub fn stores_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.data_context.stores_microservice_healthcheck()?;
        Ok(())
    }

    pub fn orders_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.data_context.orders_microservice_healthcheck()?;
        Ok(())
    }

    pub fn warehouses_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.data_context.warehouses_microservice_healthcheck()?;
        Ok(())
    }

    pub fn billing_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.data_context.billing_microservice_healthcheck()?;
        Ok(())
    }

    pub fn notifications_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.data_context.notifications_microservice_healthcheck()?;
        Ok(())
    }

    pub fn delivery_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.data_context.delivery_microservice_healthcheck()?;
        Ok(())
    }

    pub fn saga_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.data_context.saga_microservice_healthcheck()?;
        Ok(())
    }

    pub fn gateway_microservice_healthcheck(&self) -> Result<(), FailureError> {
        self.data_context.gateway_microservice_healthcheck()?;
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
            .post(&self.data_context.graphql_url())
            .header("Currency", self.currency.as_str())
            .header("FiatCurrency", self.fiat_currency.as_str());
        if let Some(ref bearer) = self.bearer {
            request = request.header("Authorization", format!("Bearer {}", bearer));
        }
        let mut res = request.json(&data).send()?;
        let result = res.json()?;
        Ok(result)
    }
}
