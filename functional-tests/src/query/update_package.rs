use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/update_package.graphql"
)]
pub struct UpdatePackageMutation;

pub use self::update_package_mutation::*;

pub type GraphqlRequestOutput = RustUpdatePackageUpdatePackage;

pub fn default_graphql_request_input() -> UpdatePackagesInput {
    UpdatePackagesInput {
        id: "".to_string(),
        client_mutation_id: "".to_string(),
        name: None,
        max_size: None,
        min_size: None,
        max_weight: None,
        min_weight: None,
        deliveries_to: None,
    }
}

impl GraphqlRequest for UpdatePackagesInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.update_package),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<UpdatePackagesInput> for serde_json::Value {
    fn from(val: UpdatePackagesInput) -> serde_json::Value {
        let request_body = UpdatePackageMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize UpdatePackageInput")
    }
}
