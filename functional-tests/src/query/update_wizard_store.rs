use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/update_wizard_store.graphql",
    response_derives = "Clone, Debug, PartialEq"
)]
pub struct UpdateWizardStoreMutation;

pub use self::update_wizard_store_mutation::*;

pub fn default_update_wizard_store_input() -> UpdateWizardStoreInput {
    UpdateWizardStoreInput {
        client_mutation_id: "".to_string(),
        store_id: None,
        name: None,
        short_description: None,
        default_language: None,
        slug: None,
        address_full: default_address_input(),
    }
}

pub fn default_address_input() -> AddressInput {
    AddressInput {
        value: None,
        country: None,
        country_code: None,
        administrative_area_level1: None,
        administrative_area_level2: None,
        locality: None,
        political: None,
        postal_code: None,
        route: None,
        street_number: None,
        place_id: None,
    }
}

pub type GraphqlRequestOutput = RustUpdateWizardStoreUpdateWizardStore;

impl GraphqlRequest for UpdateWizardStoreInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.update_wizard_store),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<UpdateWizardStoreInput> for serde_json::Value {
    fn from(val: UpdateWizardStoreInput) -> serde_json::Value {
        let request_body = UpdateWizardStoreMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize UpdateWizardStoreInput")
    }
}
