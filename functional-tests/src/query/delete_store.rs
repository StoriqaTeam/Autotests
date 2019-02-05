use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/delete_store.graphql"
)]
pub struct DeleteStoreMutation;

pub use self::delete_store_mutation::*;

pub struct DeleteStoreInput {
    pub store_id: i64,
}

pub type GraphqlRequestOutput = RustDeleteStoreDeleteStore;

impl GraphqlRequest for DeleteStoreInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.delete_store),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<DeleteStoreInput> for serde_json::Value {
    fn from(val: DeleteStoreInput) -> serde_json::Value {
        let request_body = DeleteStoreMutation::build_query(Variables {
            store_id: val.store_id,
        });
        serde_json::to_value(request_body).expect("failed to serialize DeleteStoreInput")
    }
}
