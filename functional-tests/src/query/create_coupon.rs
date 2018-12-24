use chrono::{DateTime, Utc};
use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/create_coupon.graphql"
)]
pub struct CreateCouponMutation;

pub use self::create_coupon_mutation::*;

pub type GraphqlRequestOutput = RustCreateCouponCreateCoupon;
type DateTimeUtc = DateTime<Utc>;

pub fn default_create_coupon_input() -> NewCouponInput {
    NewCouponInput {
        client_mutation_id: "".to_string(),
        code: "".to_string(),
        title: "".to_string(),
        store_id: 0,
        scope: CouponScope::STORE,
        percent: 1,
        quantity: 1,
        expired_at: None,
    }
}

impl GraphqlRequest for NewCouponInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.create_coupon),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<NewCouponInput> for serde_json::Value {
    fn from(val: NewCouponInput) -> serde_json::Value {
        let request_body = CreateCouponMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize CreateCouponInput")
    }
}
