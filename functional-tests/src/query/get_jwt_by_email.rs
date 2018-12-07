#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/get_jwt_by_email.graphql",
)]
pub struct GetJwtByEmailMutation;

pub use self::get_jwt_by_email_mutation::*;

pub fn default_create_jwt_email_input() -> CreateJWTEmailInput {
    CreateJWTEmailInput {
        client_mutation_id: "".to_string(),
        email: "user@mail.com".to_string(),
        password: "Qwerty123".to_string(),
    }
}
