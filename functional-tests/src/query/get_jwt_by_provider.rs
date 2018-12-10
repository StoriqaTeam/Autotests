#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/get_jwt_by_provider.graphql"
)]
pub struct GetJwtByProviderMutation;

pub use self::get_jwt_by_provider_mutation::*;

pub fn facebook_create_jwt_provider_input() -> CreateJWTProviderInput {
    CreateJWTProviderInput {
        client_mutation_id: "".to_string(),
        provider: Provider::FACEBOOK,
        token: "facebook-token".to_string(),
        additional_data: None,
    }
}

pub fn google_create_jwt_provider_input() -> CreateJWTProviderInput {
    CreateJWTProviderInput {
        client_mutation_id: "".to_string(),
        provider: Provider::GOOGLE,
        token: "google-token".to_string(),
        additional_data: None,
    }
}
