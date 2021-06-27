#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(oneof="request::Command", tags="1, 2")]
    pub command: ::core::option::Option<request::Command>,
}
/// Nested message and enum types in `Request`.
pub mod request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Command {
        #[prost(message, tag="1")]
        Get(super::RequestGet),
        #[prost(message, tag="2")]
        Put(super::RequestPut),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestGet {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPut {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
