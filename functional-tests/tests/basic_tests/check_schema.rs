use failure::Error as FailureError;

use graphql_client::Response;
use serde_json;

use functional_tests::context::TestContext;
use functional_tests::query::*;

#[test]
pub fn check_schema() {
    let context = TestContext::new();

    let response_data = context
        .request(introspection::IntrospectionInput)
        .expect("Cannot get schema");

    let expect_result: Response<introspection::ResponseData> =
        serde_json::from_str(introspection::INTROSPECTION_RESPONSE).unwrap();
    let expect_data = expect_result
        .data
        .expect("Expected introspection schema is none.");

    assert!(response_data == expect_data);
}
