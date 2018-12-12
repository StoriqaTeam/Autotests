#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/delete_delivery_company.graphql"
)]
pub struct DeleteCompanyMutation;

pub use self::delete_company_mutation::*;
