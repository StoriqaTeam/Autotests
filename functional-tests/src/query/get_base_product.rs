use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/get_base_product.graphql",
    response_derives = "Debug, PartialEq, Clone"
)]
pub struct GetBaseProductQuery;

#[derive(Serialize, Debug, PartialEq, Copy, Clone)]
pub enum Visibility {
    Published,
    Active,
}

pub use self::get_base_product_query::*;

pub struct GetBaseProductInput {
    pub base_product_id: i64,
    pub visibility: Option<Visibility>,
}

impl Default for GetBaseProductInput {
    fn default() -> Self {
        Self {
            base_product_id: 0,
            visibility: Some(Visibility::Active),
        }
    }
}

pub type GraphqlRequestOutput = Option<RustGetBaseProductBaseProduct>;

impl GraphqlRequest for GetBaseProductInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.base_product),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<GetBaseProductInput> for serde_json::Value {
    fn from(val: GetBaseProductInput) -> serde_json::Value {
        let request_body = GetBaseProductQuery::build_query(Variables {
            id: val.base_product_id,
            visibility: val.visibility,
        });
        serde_json::to_value(request_body).expect("failed to serialize GetBaseProductInput")
    }
}
