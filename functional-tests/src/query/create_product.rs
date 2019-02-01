use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/create_product.graphql",
    response_derives = "Debug, PartialEq, Clone"
)]
pub struct CreateProductMutation;

pub use self::create_product_mutation::*;

pub fn default_create_product_input() -> CreateProductWithAttributesInput {
    CreateProductWithAttributesInput {
        client_mutation_id: "".to_string(),
        product: default_new_product(),
        attributes: vec![],
    }
}

pub fn default_new_product() -> NewProduct {
    NewProduct {
        client_mutation_id: None,
        base_product_id: None,
        discount: None,
        photo_main: None,
        additional_photos: None,
        vendor_code: "1".to_string(),
        cashback: None,
        price: 50000f64, // TODO: validate > 0.0 need test
        pre_order: None,
        pre_order_days: None,
    }
}

type GraphqlRequestOutput = RustCreateProductCreateProduct;

impl GraphqlRequest for CreateProductWithAttributesInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.create_product),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<CreateProductWithAttributesInput> for serde_json::Value {
    fn from(val: CreateProductWithAttributesInput) -> serde_json::Value {
        let request_body = CreateProductMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize CreateProductMutation")
    }
}
