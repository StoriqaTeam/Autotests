#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/update_store.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct UpdateStoreMutation;

pub use self::update_store_mutation::*;

pub fn default_update_store_input() -> UpdateStoreInput {
    UpdateStoreInput {
        client_mutation_id: "".to_string(),
        id: "".to_string(),
        name: Some(vec![TranslationInput {
            lang: Language::EN,
            text: "updated name".to_string(),
        }]),
        short_description: Some(vec![TranslationInput {
            lang: Language::EN,
            text: "updated short description".to_string(),
        }]),
        slug: Some("updatedstoreslug".to_string()),
        cover: Some("updated cover".to_string()),
        logo: Some("updated logo".to_string()),
        phone: Some("1234567".to_string()),
        email: Some("updated@email.com".to_string()),
        long_description: Some(vec![TranslationInput {
            lang: Language::EN,
            text: "updated long description".to_string(),
        }]),
        instagram_url: Some("updated instagram url".to_string()),
        twitter_url: Some("updated twitter url".to_string()),
        facebook_url: Some("updated facebook url".to_string()),
        default_language: Some(Language::CH),
        slogan: Some("updated slogan".to_string()),
        rating: Some(1.0),
        address_full: AddressInput {
            value: Some("updated address value".to_string()),
            country: Some("Updated".to_string()),
            country_code: Some("UPD".to_string()),
            administrative_area_level1: Some("updated administrative_area_level1".to_string()),
            administrative_area_level2: Some("administrative_area_level2".to_string()),
            locality: Some("updated locality".to_string()),
            political: Some("updated political".to_string()),
            postal_code: Some("updated postal_code".to_string()),
            route: Some("updated route".to_string()),
            street_number: Some("updated street_number".to_string()),
            place_id: Some("updated place_id".to_string()),
        },
    }
}
