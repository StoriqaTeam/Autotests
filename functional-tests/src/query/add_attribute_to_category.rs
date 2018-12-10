#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/add_attribute_to_category.graphql"
)]
pub struct AddAttributeToCategoryMutation;

pub use self::add_attribute_to_category_mutation::*;

pub fn default_add_attribute_to_categoryinput() -> AddAttributeToCategoryInput {
    AddAttributeToCategoryInput {
        client_mutation_id: "".to_string(),
        cat_id: 0,
        attr_id: 0,
    }
}
