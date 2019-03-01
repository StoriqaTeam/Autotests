use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/get_jwt_by_provider.graphql"
)]
pub struct GetJwtByProviderMutation;

pub use self::get_jwt_by_provider_mutation::*;

pub fn facebook_create_jwt_provider_input() -> CreateJWTProviderInput {
    CreateJWTProviderInput {
        client_mutation_id: "".to_string(),
        provider: Provider::FACEBOOK,
        token: "facebook-token".to_string(),
        additional_data: None,
    }
}

pub fn google_create_jwt_provider_input() -> CreateJWTProviderInput {
    CreateJWTProviderInput {
        client_mutation_id: "".to_string(),
        provider: Provider::GOOGLE,
        token: "google-token".to_string(),
        additional_data: None,
    }
}

pub fn new_user_additional_data_input() -> NewUserAdditionalDataInput {
    NewUserAdditionalDataInput {
        country: Some("MM".to_string()),
        referal: Some(1),
        referer: Some("localhost".to_string()),
        utm_marks: Some(vec![UtmMarkInput {
            key: "source".to_string(),
            value: "word_of_mouth".to_string(),
        }]),
    }
}

type GraphqlRequestOutput = RustGetJwtByProviderGetJwtByProvider;

impl GraphqlRequest for CreateJWTProviderInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.get_jwt_by_provider),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<CreateJWTProviderInput> for serde_json::Value {
    fn from(val: CreateJWTProviderInput) -> serde_json::Value {
        let request_body = GetJwtByProviderMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize CreateJWTProviderInput")
    }
}
