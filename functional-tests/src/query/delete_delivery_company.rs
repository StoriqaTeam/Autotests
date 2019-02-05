use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/delete_delivery_company.graphql"
)]
pub struct DeleteCompanyMutation;

pub use self::delete_company_mutation::*;

pub struct DeleteCompanyInput {
    pub company_id: i64,
}

pub type GraphqlRequestOutput = RustDeleteCompanyDeleteCompany;

impl GraphqlRequest for DeleteCompanyInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.delete_company),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<DeleteCompanyInput> for serde_json::Value {
    fn from(val: DeleteCompanyInput) -> serde_json::Value {
        let request_body = DeleteCompanyMutation::build_query(Variables { id: val.company_id });
        serde_json::to_value(request_body).expect("failed to serialize DeleteCompanyInput")
    }
}
