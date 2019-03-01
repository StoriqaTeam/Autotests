use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/generate_coupon_code.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct GenerateCouponCodeQuery;

pub use self::generate_coupon_code_query::*;

pub struct GenerateCouponCodeInput;

pub type GraphqlRequestOutput = String;

impl GraphqlRequest for GenerateCouponCodeInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.generate_coupon_code),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<GenerateCouponCodeInput> for serde_json::Value {
    fn from(_val: GenerateCouponCodeInput) -> serde_json::Value {
        let request_body = GenerateCouponCodeQuery::build_query(Variables {});
        serde_json::to_value(request_body).expect("failed to serialize GenerateCouponCodeInput")
    }
}
