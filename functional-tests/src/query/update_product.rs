use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/update_product.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct UpdateProductMutation;

pub use self::update_product_mutation::*;

pub fn default_update_product_with_attributes_input() -> UpdateProductWithAttributesInput {
    UpdateProductWithAttributesInput {
        client_mutation_id: "".to_string(),
        id: "".to_string(),
        product: Some(default_update_product_input()),
        attributes: None,
    }
}

pub fn default_update_product_input() -> UpdateProduct {
    UpdateProduct {
        discount: None,
        photo_main: None,
        additional_photos: None,
        vendor_code: None,
        cashback: None,
        price: None,
        pre_order: None,
        pre_order_days: None,
    }
}

type GraphqlRequestOutput = RustUpdateProductUpdateProduct;

impl GraphqlRequest for UpdateProductWithAttributesInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.update_product),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<UpdateProductWithAttributesInput> for serde_json::Value {
    fn from(val: UpdateProductWithAttributesInput) -> serde_json::Value {
        let request_body = UpdateProductMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body)
            .expect("failed to serialize UpdateProductWithAttributesInput")
    }
}
