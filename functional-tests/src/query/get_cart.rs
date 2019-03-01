use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/get_cart.graphql",
    response_derives = "Clone, Debug, PartialEq"
)]
pub struct GetCartQuery;

pub use self::get_cart_query::*;

pub struct GetCartInput {
    currency_type: Option<CurrencyType>,
}

pub fn default_get_cart_input() -> GetCartInput {
    GetCartInput {
        currency_type: Some(CurrencyType::CRYPTO),
    }
}

type GraphqlRequestOutput = Option<RustGetCartCart>;

impl RustGetCartCart {
    pub fn get_product(self, product_id: i64) -> Option<RustGetCartCartStoresEdgesNodeProducts> {
        self.stores
            .edges
            .into_iter()
            .flat_map(|e| e.node.products)
            .find(|product| product.raw_id == product_id)
    }

    pub fn get_store(self, store_id: i64) -> Option<RustGetCartCartStoresEdgesNode> {
        self.stores
            .edges
            .into_iter()
            .map(|e| e.node)
            .find(|store| store.raw_id == store_id)
    }

    pub fn get_products(self) -> Vec<RustGetCartCartStoresEdgesNodeProducts> {
        self.stores
            .edges
            .into_iter()
            .map(|edge| edge.node.products)
            .flatten()
            .collect()
    }
}

impl GraphqlRequest for GetCartInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;

        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.cart),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<GetCartInput> for serde_json::Value {
    fn from(val: GetCartInput) -> serde_json::Value {
        let request_body = GetCartQuery::build_query(Variables {
            currency_type: val.currency_type,
        });
        serde_json::to_value(request_body).expect("failed to serialize GetCartInput")
    }
}
