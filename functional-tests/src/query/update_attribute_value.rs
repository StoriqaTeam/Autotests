use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/update_attribute_value.graphql"
)]
pub struct UpdateAttributeValueMutation;

pub use self::update_attribute_value_mutation::*;

pub fn default_create_attribute_value_input() -> UpdateAttributeValueInput {
    UpdateAttributeValueInput {
        raw_id: 0,
        client_mutation_id: "".to_string(),
        raw_attribute_id: 0,
        code: Some("update_attribute_value_code".to_string()),
        translations: Some(vec![
            TranslationInput {
                lang: Language::EN,
                text: "update attribute value".to_string(),
            },
            TranslationInput {
                lang: Language::CH,
                text: "update attribute value china".to_string(),
            },
        ]),
    }
}

type GraphqlRequestOutput = RustUpdateAttributeValueUpdateAttributeValue;

impl GraphqlRequest for UpdateAttributeValueInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.update_attribute_value),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<UpdateAttributeValueInput> for serde_json::Value {
    fn from(val: UpdateAttributeValueInput) -> serde_json::Value {
        let request_body = UpdateAttributeValueMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize UpdateAttributeValueInput")
    }
}
