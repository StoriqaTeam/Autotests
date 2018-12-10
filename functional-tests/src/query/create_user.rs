#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/create_user.graphql"
)]
pub struct CreateUserMutation;

pub use self::create_user_mutation::*;

pub fn default_create_user_input() -> CreateUserInput {
    CreateUserInput {
        additional_data: None,
        client_mutation_id: "".to_string(),
        device: None,
        email: "user@mail.com".to_string(),
        first_name: "User".to_string(),
        last_name: "Userovsky".to_string(),
        password: "Qwerty123".to_string(),
        project: None,
    }
}
