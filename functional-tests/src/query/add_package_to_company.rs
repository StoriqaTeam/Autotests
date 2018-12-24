use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/add_package_to_company.graphql"
)]
pub struct AddPackageToCompanyMutation;

pub use self::add_package_to_company_mutation::*;

pub type GraphqlRequestOutput = RustAddPackageToCompanyAddPackageToCompany;

pub fn default_add_package_to_company_input() -> NewCompaniesPackagesInput {
    NewCompaniesPackagesInput {
        client_mutation_id: "".to_string(),
        company_id: 0,
        package_id: 0,
        dimensional_factor: None,
        uses_static_rates: None,
    }
}

impl GraphqlRequest for NewCompaniesPackagesInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.add_package_to_company),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<NewCompaniesPackagesInput> for serde_json::Value {
    fn from(val: NewCompaniesPackagesInput) -> serde_json::Value {
        let request_body = AddPackageToCompanyMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize AddPackageToCompanyInput")
    }
}
