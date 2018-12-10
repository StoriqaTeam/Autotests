use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;

use config::Config;
use microservice::{StoresMicroservice, UsersMicroservice};
use query::*;

pub struct TestContext {
    bearer: Option<String>,
    client: Client,
    config: Config,
    users_microservice: UsersMicroservice,
    stores_microservice: StoresMicroservice,
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
            config,
            bearer: None,
            client: client.clone(),
        };

        context.clear_all_data().unwrap();
        context
    }

    pub fn verify_email(&self, email: &str) -> Result<(), FailureError> {
        self.users_microservice.verify_email(email)
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

    graphql_request!(
        create_category,
        create_category,
        CreateCategoryInput,
        CreateCategoryMutation
    );
    graphql_request!(
        create_user,
        create_user,
        CreateUserInput,
        CreateUserMutation
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
        create_store,
        create_store,
        CreateStoreInput,
        CreateStoreMutation
    );
    graphql_request!(
        create_attribute,
        create_attribute,
        CreateAttributeInput,
        CreateAttributeMutation
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
