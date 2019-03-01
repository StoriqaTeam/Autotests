use failure::Error as FailureError;

pub trait GraphqlRequest: Into<serde_json::Value> {
    type Output;
    fn response(body: serde_json::Value) -> Result<Self::Output, FailureError>;
}
