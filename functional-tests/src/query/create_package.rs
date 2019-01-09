use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/create_package.graphql"
)]
pub struct CreatePackageMutation;

pub use self::create_package_mutation::*;

pub type GraphqlRequestOutput = RustCreatePackageCreatePackage;

pub fn default_create_package_input() -> NewPackagesInput {
    NewPackagesInput {
        client_mutation_id: "".to_string(),
        name: "".to_string(),
        max_size: 1000,
        min_size: 100,
        max_weight: 3000,
        min_weight: 300,
        deliveries_to: vec!["USA".to_string(), "GBR".to_string()],
    }
}

impl GraphqlRequest for NewPackagesInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.create_package),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<NewPackagesInput> for serde_json::Value {
    fn from(val: NewPackagesInput) -> serde_json::Value {
        let request_body = CreatePackageMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize CreatePackageInput")
    }
}
