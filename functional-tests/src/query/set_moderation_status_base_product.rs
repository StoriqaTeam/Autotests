use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/set_moderation_status_base_product.graphql"
)]
pub struct SetModerationStatusBaseProductMutation;

pub use self::set_moderation_status_base_product_mutation::*;

pub fn default_set_moderation_status_base_product_input() -> BaseProductModerateInput {
    BaseProductModerateInput {
        id: "".to_string(),
        status: Status::PUBLISHED,
    }
}

type GraphqlRequestOutput = RustSetModerationStatusBaseProductSetModerationStatusBaseProduct;

impl GraphqlRequest for BaseProductModerateInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.set_moderation_status_base_product),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<BaseProductModerateInput> for serde_json::Value {
    fn from(val: BaseProductModerateInput) -> serde_json::Value {
        let request_body =
            SetModerationStatusBaseProductMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize BaseProductModerateInput")
    }
}
