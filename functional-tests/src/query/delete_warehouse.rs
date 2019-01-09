use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/delete_warehouse.graphql",
    response_derives = "Clone, Debug, PartialEq"
)]
pub struct DeleteWarehouseMutation;

pub use self::delete_warehouse_mutation::*;

pub struct DeleteWarehouseInput {
    pub id: String,
}

pub type GraphqlRequestOutput = Option<RustDeleteWarehouseDeleteWarehouse>;

impl GraphqlRequest for DeleteWarehouseInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.delete_warehouse),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<DeleteWarehouseInput> for serde_json::Value {
    fn from(val: DeleteWarehouseInput) -> serde_json::Value {
        let request_body = DeleteWarehouseMutation::build_query(Variables { id: val.id });
        serde_json::to_value(request_body).expect("failed to serialize DeleteWarehouseInput")
    }
}
