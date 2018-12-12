#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/update_user.graphql"
)]
pub struct UpdateUserMutation;

pub use self::update_user_mutation::*;

pub fn default_update_user_input() -> UpdateUserInput {
    UpdateUserInput {
        client_mutation_id: "".to_string(),
        id: "".to_string(),
        is_active: None,
        phone: Some("9876543".to_string()),
        first_name: Some("updated user".to_string()),
        last_name: Some("updated userovsky".to_string()),
        middle_name: Some("updated middleName".to_string()),
        gender: Some(Gender::MALE),
        birthdate: Some("1990-01-01".to_string()),
        avatar: Some("updated avatar".to_string()),
    }
}
