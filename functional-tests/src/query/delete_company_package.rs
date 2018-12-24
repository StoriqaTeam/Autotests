use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/delete_company_package.graphql"
)]
pub struct DeleteCompanyPackageMutation;

pub use self::delete_company_package_mutation::*;

pub type GraphqlRequestOutput = RustDeleteCompanyPackageDeleteCompanyPackage;

pub struct DeleteCompanyPackageInput {
    pub company_id: i64,
    pub package_id: i64,
}

impl GraphqlRequest for DeleteCompanyPackageInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.delete_company_package),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<DeleteCompanyPackageInput> for serde_json::Value {
    fn from(val: DeleteCompanyPackageInput) -> serde_json::Value {
        let request_body = DeleteCompanyPackageMutation::build_query(Variables {
            company_id: val.company_id,
            package_id: val.package_id,
        });
        serde_json::to_value(request_body).expect("failed to serialize DeleteCompanyPackageInput")
    }
}
