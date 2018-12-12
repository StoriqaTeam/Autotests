#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/verify_email.graphql"
)]
pub struct VerifyEmailMutation;

pub use self::verify_email_mutation::*;

pub fn default_verify_email_input() -> VerifyEmailApply {
    VerifyEmailApply {
        client_mutation_id: "".to_string(),
        token: "".to_string(),
        project: Some(Project::MARKET_PLACE),
    }
}
