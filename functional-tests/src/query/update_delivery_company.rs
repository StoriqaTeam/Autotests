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
