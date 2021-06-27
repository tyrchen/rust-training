/// naive block
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// calc
    #[prost(bytes = "vec", tag = "2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// tune nonce to get hash with x 0 prefix -> 0x000abcdfadbd
    #[prost(uint32, tag = "3")]
    pub nonce: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockStatus {
    #[prost(uint32, tag = "1")]
    pub code: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientInfo {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHash {
    /// unique id for the block
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    /// PoW hash
    #[prost(bytes = "vec", tag = "2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// nonce
    #[prost(uint32, tag = "3")]
    pub nonce: u32,
}
#[doc = r" Generated client implementations."]
pub mod pow_builder_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct PowBuilderClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PowBuilderClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PowBuilderClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn subscribe(
            &mut self,
            request: impl tonic::IntoRequest<super::ClientInfo>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::BlockHash>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.PowBuilder/Subscribe");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn submit(
            &mut self,
            request: impl tonic::IntoRequest<super::Block>,
        ) -> Result<tonic::Response<super::BlockStatus>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/abi.PowBuilder/Submit");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for PowBuilderClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for PowBuilderClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PowBuilderClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod pow_builder_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with PowBuilderServer."]
    #[async_trait]
    pub trait PowBuilder: Send + Sync + 'static {
        #[doc = "Server streaming response type for the Subscribe method."]
        type SubscribeStream: futures_core::Stream<Item = Result<super::BlockHash, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn subscribe(
            &self,
            request: tonic::Request<super::ClientInfo>,
        ) -> Result<tonic::Response<Self::SubscribeStream>, tonic::Status>;
        async fn submit(
            &self,
            request: tonic::Request<super::Block>,
        ) -> Result<tonic::Response<super::BlockStatus>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct PowBuilderServer<T: PowBuilder> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: PowBuilder> PowBuilderServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for PowBuilderServer<T>
    where
        T: PowBuilder,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/abi.PowBuilder/Subscribe" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeSvc<T: PowBuilder>(pub Arc<T>);
                    impl<T: PowBuilder> tonic::server::ServerStreamingService<super::ClientInfo> for SubscribeSvc<T> {
                        type Response = super::BlockHash;
                        type ResponseStream = T::SubscribeStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClientInfo>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).subscribe(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = SubscribeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/abi.PowBuilder/Submit" => {
                    #[allow(non_camel_case_types)]
                    struct SubmitSvc<T: PowBuilder>(pub Arc<T>);
                    impl<T: PowBuilder> tonic::server::UnaryService<super::Block> for SubmitSvc<T> {
                        type Response = super::BlockStatus;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Block>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).submit(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SubmitSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: PowBuilder> Clone for PowBuilderServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: PowBuilder> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PowBuilder> tonic::transport::NamedService for PowBuilderServer<T> {
        const NAME: &'static str = "abi.PowBuilder";
    }
}
