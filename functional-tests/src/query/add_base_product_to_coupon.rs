use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/add_base_product_to_coupon.graphql"
)]
pub struct AddBaseProductToCouponMutation;

pub use self::add_base_product_to_coupon_mutation::*;

pub fn default_add_base_product_to_coupon_input() -> ChangeBaseProductsInCoupon {
    ChangeBaseProductsInCoupon {
        client_mutation_id: "".to_string(),
        raw_id: 0,
        raw_base_product_id: 0,
    }
}

type GraphqlRequestOutput = RustAddBaseProductToCouponAddBaseProductToCoupon;

impl GraphqlRequest for ChangeBaseProductsInCoupon {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.add_base_product_to_coupon),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<ChangeBaseProductsInCoupon> for serde_json::Value {
    fn from(val: ChangeBaseProductsInCoupon) -> serde_json::Value {
        let request_body = AddBaseProductToCouponMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize ChangeBaseProductsInCoupon")
    }
}
