use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
schema_path = "graphql/schema.json",
query_path = "graphql/queries/delete_package.graphql"
)]
pub struct DeletePackageMutation;

pub use self::delete_package_mutation::*;

pub struct DeletePackagesInput { pub id: i64 }

impl GraphqlRequest for DeletePackagesInput {
    type Output = RustDeletePackageDeletePackage;

    fn response(body: serde_json::Value) -> Result<RustDeletePackageDeletePackage, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.delete_package),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<DeletePackagesInput> for serde_json::Value {
    fn from(val: DeletePackagesInput) -> serde_json::Value {
        let request_body = DeletePackageMutation::build_query(Variables { id: val.id });
        serde_json::to_value(request_body).expect("failed to serialize DeletePackageInput")
    }
}
