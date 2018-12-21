use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
schema_path = "graphql/schema.json",
query_path = "graphql/queries/update_warehouse.graphql"
)]
pub struct UpdateWarehouseMutation;

pub use self::update_warehouse_mutation::*;

pub fn default_update_warehouse_input() -> UpdateWarehouseInput {
    UpdateWarehouseInput {
        id: "".to_string(),
        client_mutation_id: "".to_string(),
        address_full: default_address_input(),
        location: None,
        name: None,
        slug: None
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
        place_id: None
    }
}

pub fn default_geo_point_input() -> GeoPointInput {
    GeoPointInput {
        x: 0.0,
        y: 0.0
    }
}

impl GraphqlRequest for UpdateWarehouseInput {
    type Output = Option<RustUpdateWarehouseUpdateWarehouse>;

    fn response(body: serde_json::Value) -> Result<Option<RustUpdateWarehouseUpdateWarehouse>, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.update_warehouse),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<UpdateWarehouseInput> for serde_json::Value {
    fn from(val: UpdateWarehouseInput) -> serde_json::Value {
        let request_body = UpdateWarehouseMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize UpdateWarehouseInput")
    }
}
