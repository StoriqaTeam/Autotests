use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/get_cart_v2.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct GetCartV2Query;

pub use self::get_cart_v2_query::*;

pub struct GetCartV2Input {
    user_country_code: String,
}

pub fn default_get_cart_v2_input() -> GetCartV2Input {
    GetCartV2Input {
        user_country_code: "RUS".to_string(),
    }
}

type GraphqlRequestOutput = Option<RustGetCartV2CartV2>;

impl RustGetCartV2CartV2 {
    pub fn get_product(
        self,
        product_id: i64,
    ) -> Option<RustGetCartV2CartV2StoresEdgesNodeProducts> {
        self.stores
            .edges
            .into_iter()
            .flat_map(|e| e.node.products)
            .find(|product| product.raw_id == product_id)
    }
}

impl GraphqlRequest for GetCartV2Input {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;

        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.cart_v2),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<GetCartV2Input> for serde_json::Value {
    fn from(val: GetCartV2Input) -> serde_json::Value {
        let request_body = GetCartV2Query::build_query(Variables {
            user_country_code: val.user_country_code,
        });
        serde_json::to_value(request_body).expect("failed to serialize GetCartV2Input")
    }
}
