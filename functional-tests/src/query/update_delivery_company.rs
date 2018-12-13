use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/update_delivery_company.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct UpdateCompanyMutation;

pub use self::update_company_mutation::*;

pub fn default_update_company_input() -> UpdateCompanyInput {
    UpdateCompanyInput {
        client_mutation_id: "".to_string(),
        id: "".to_string(),
        name: None,
        label: None,
        description: None,
        deliveries_from: None,
        currency: None,
        logo: None,
    }
}

type GraphqlRequestOutput = RustUpdateCompanyUpdateCompany;

impl GraphqlRequest for UpdateCompanyInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.update_company),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<UpdateCompanyInput> for serde_json::Value {
    fn from(val: UpdateCompanyInput) -> serde_json::Value {
        let request_body = UpdateCompanyMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize UpdateCompanyInput")
    }
}
