use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/get_store.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct GetStoreQuery;

#[derive(Serialize, Debug, PartialEq)]
pub enum Visibility {
    Published,
    Active,
}

pub use self::get_store_query::*;

pub struct GetStoreInput {
    pub store_id: i64,
    pub visibility: Option<Visibility>,
}

impl Default for GetStoreInput {
    fn default() -> Self {
        Self {
            store_id: 0,
            visibility: Some(Visibility::Active),
        }
    }
}

pub type GraphqlRequestOutput = Option<RustGetStoreStore>;

impl GraphqlRequest for GetStoreInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.store),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<GetStoreInput> for serde_json::Value {
    fn from(val: GetStoreInput) -> serde_json::Value {
        let request_body = GetStoreQuery::build_query(Variables {
            id: val.store_id,
            visibility: val.visibility,
        });
        serde_json::to_value(request_body).expect("failed to serialize GetStoreInput")
    }
}
