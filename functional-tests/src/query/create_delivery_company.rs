use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/create_delivery_company.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct CreateCompanyMutation;

pub use self::create_company_mutation::*;

pub fn default_create_company_input() -> NewCompanyInput {
    NewCompanyInput {
        client_mutation_id: "".to_string(),
        name: "".to_string(),
        label: "".to_string(),
        description: None,
        deliveries_from: vec![],
        currency: Currency::STQ,
        logo: "".to_string(),
    }
}

impl Clone for NewCompanyInput {
    fn clone(&self) -> Self {
        Self {
            client_mutation_id: self.client_mutation_id.clone(),
            name: self.name.clone(),
            label: self.label.clone(),
            description: self.description.clone(),
            deliveries_from: self.deliveries_from.clone(),
            currency: self.currency.clone(),
            logo: self.logo.clone(),
        }
    }
}

impl Clone for Currency {
    fn clone(&self) -> Self {
        match self {
            &Currency::RUB => Currency::RUB,
            &Currency::EUR => Currency::EUR,
            &Currency::USD => Currency::USD,
            &Currency::BTC => Currency::BTC,
            &Currency::ETH => Currency::ETH,
            &Currency::STQ => Currency::STQ,
            &Currency::Other(ref value) => Currency::Other(value.clone()),
        }
    }
}

pub type GraphqlRequestOutput = RustCreateCompanyCreateCompany;

impl GraphqlRequest for NewCompanyInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.create_company),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<NewCompanyInput> for serde_json::Value {
    fn from(val: NewCompanyInput) -> serde_json::Value {
        let request_body = CreateCompanyMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize CreateCompanyMutation")
    }
}
