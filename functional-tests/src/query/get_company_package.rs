use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
schema_path = "graphql/schema.json",
query_path = "graphql/queries/get_company_package.graphql",
response_derives = "Debug, PartialEq"
)]
pub struct GetCompanyPackageQuery;

pub use self::get_company_package_query::*;

pub struct GetCompanyPackageInput { pub id: i64 }
pub type GraphqlRequestOutput = Option<RustGetCompanyPackageCompanyPackage>;

impl GraphqlRequest for GetCompanyPackageInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.company_package),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<GetCompanyPackageInput> for serde_json::Value {
    fn from(val: GetCompanyPackageInput) -> serde_json::Value {
        let request_body = GetCompanyPackageQuery::build_query(Variables { id: val.id });
        serde_json::to_value(request_body).expect("failed to serialize GetCompanyPackageInput")
    }
}
