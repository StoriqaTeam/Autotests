use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/deactivate_product.graphql"
)]
pub struct DeactivateProductMutation;

pub use self::deactivate_product_mutation::*;

pub fn default_deactivate_product_input() -> DeactivateProductInput {
    DeactivateProductInput {
        client_mutation_id: "".to_string(),
        id: "".to_string(),
    }
}

type GraphqlRequestOutput = RustDeactivateProductDeactivateProduct;

impl GraphqlRequest for DeactivateProductInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.deactivate_product),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<DeactivateProductInput> for serde_json::Value {
    fn from(val: DeactivateProductInput) -> serde_json::Value {
        let request_body = DeactivateProductMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize DeactivateProductInput")
    }
}
