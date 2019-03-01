use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/get_available_shipping_for_user.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct GetAvailableShippingForUserQuery;

pub use self::get_available_shipping_for_user_query::*;

pub struct GetAvailableShippingForUserInput {
    pub user_country_code: String,
    pub base_product_id: i64,
}

pub fn default_get_available_shipping_for_user_input() -> GetAvailableShippingForUserInput {
    GetAvailableShippingForUserInput {
        user_country_code: "RUS".to_string(),
        base_product_id: 0,
    }
}

type GraphqlRequestOutput = RustAvailableShippingForUserAvailableShippingForUser;

impl GraphqlRequest for GetAvailableShippingForUserInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.available_shipping_for_user),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<GetAvailableShippingForUserInput> for serde_json::Value {
    fn from(val: GetAvailableShippingForUserInput) -> serde_json::Value {
        let request_body = GetAvailableShippingForUserQuery::build_query(Variables {
            user_country: val.user_country_code,
            base_product_id: val.base_product_id,
        });
        serde_json::to_value(request_body)
            .expect("failed to serialize GetAvailableShippingForUserInput")
    }
}
