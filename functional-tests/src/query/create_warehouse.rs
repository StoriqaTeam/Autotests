use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
schema_path = "graphql/schema.json",
query_path = "graphql/queries/create_warehouse.graphql"
)]
pub struct CreateWarehouseMutation;

pub use self::create_warehouse_mutation::*;

pub type GraphqlRequestInput = CreateWarehouseInput;
pub type GraphqlRequestOutput = RustCreateWarehouseCreateWarehouse;

pub fn default_graphql_request_input() -> GraphqlRequestInput {
    CreateWarehouseInput {
        client_mutation_id: "".to_string(),
        address_full: default_address_input(),
        location: Some(default_geo_point_input()),
        store_id: 0,
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

impl GraphqlRequest for GraphqlRequestInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.create_warehouse),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<CreateWarehouseInput> for serde_json::Value {
    fn from(val: GraphqlRequestInput) -> serde_json::Value {
        let request_body = CreateWarehouseMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize CreateWarehouseInput")
    }
}
