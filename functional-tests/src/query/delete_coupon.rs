use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/delete_coupon.graphql",
    response_derives = "Clone, Debug, PartialEq"
)]
pub struct DeleteCouponMutation;

pub use self::delete_coupon_mutation::*;

pub type GraphqlRequestOutput = RustDeleteCouponDeleteCoupon;

pub struct DeleteCouponInput {
    pub id: i64,
}

impl GraphqlRequest for DeleteCouponInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.delete_coupon),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<DeleteCouponInput> for serde_json::Value {
    fn from(val: DeleteCouponInput) -> serde_json::Value {
        let request_body = DeleteCouponMutation::build_query(Variables { id: val.id });
        serde_json::to_value(request_body).expect("failed to serialize DeleteCouponInput")
    }
}
