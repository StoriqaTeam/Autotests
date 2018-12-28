use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/delete_wizard_store.graphql",
    response_derives = "Clone, Debug, PartialEq"
)]
pub struct DeleteWizardStoreMutation;

pub use self::delete_wizard_store_mutation::*;

pub struct DeleteWizardStoreInput;
pub type GraphqlRequestOutput = RustDeleteWizardStoreDeleteWizardStore;

impl GraphqlRequest for DeleteWizardStoreInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.delete_wizard_store),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<DeleteWizardStoreInput> for serde_json::Value {
    fn from(_val: DeleteWizardStoreInput) -> serde_json::Value {
        let request_body = DeleteWizardStoreMutation::build_query(Variables {});
        serde_json::to_value(request_body).expect("failed to serialize DeleteWizardStoreInput")
    }
}
