use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/add_in_cart_v2.graphql"
)]
pub struct AddInCartV2Mutation;

pub use self::add_in_cart_v2_mutation::*;

pub fn default_add_in_cart_v2_input() -> AddInCartInputV2 {
    AddInCartInputV2 {
        client_mutation_id: "".to_string(),
        product_id: 1,
        value: None,
        shipping_id: None,
        user_country_code: "RUS".to_string(),
    }
}

type GraphqlRequestOutput = Option<RustAddInCartV2AddInCartV2>;

impl GraphqlRequest for AddInCartInputV2 {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.add_in_cart_v2),
            (_, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<AddInCartInputV2> for serde_json::Value {
    fn from(val: AddInCartInputV2) -> serde_json::Value {
        let request_body = AddInCartV2Mutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize AddInCartInputV2")
    }
}
