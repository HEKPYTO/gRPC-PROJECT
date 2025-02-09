#[derive(Clone, PartialEq, prost::Message)]
pub struct TestRequest {
    #[prost(string, tag = "1")]
    pub message: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct TestResponse {
    #[prost(string, tag = "1")]
    pub message: String,
}
