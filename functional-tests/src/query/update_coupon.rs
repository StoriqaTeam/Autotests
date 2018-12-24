use chrono::{DateTime, Utc};
use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/update_coupon.graphql"
)]
pub struct UpdateCouponMutation;

pub use self::update_coupon_mutation::*;

pub type GraphqlRequestOutput = RustUpdateCouponUpdateCoupon;
type DateTimeUtc = DateTime<Utc>;

pub fn default_update_coupon_input() -> UpdateCouponInput {
    UpdateCouponInput {
        id: "".to_string(),
        client_mutation_id: "".to_string(),
        percent: None,
        quantity: None,
        expired_at: None,
        is_active: None,
    }
}

impl GraphqlRequest for UpdateCouponInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.update_coupon),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<UpdateCouponInput> for serde_json::Value {
    fn from(val: UpdateCouponInput) -> serde_json::Value {
        let request_body = UpdateCouponMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize UpdateCouponInput")
    }
}
