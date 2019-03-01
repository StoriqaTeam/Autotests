use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/get_wizard_store.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct GetWizardStoreQuery;

pub use self::get_wizard_store_query::*;

pub struct GetWizardStoreInput;

impl GraphqlRequest for GetWizardStoreInput {
    type Output = Option<RustGetWizardStoreMe>;

    fn response(body: serde_json::Value) -> Result<Option<RustGetWizardStoreMe>, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.me),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<GetWizardStoreInput> for serde_json::Value {
    fn from(_val: GetWizardStoreInput) -> serde_json::Value {
        let request_body = GetWizardStoreQuery::build_query(Variables {});
        serde_json::to_value(request_body).expect("failed to serialize GetWizardStoreInput")
    }
}
