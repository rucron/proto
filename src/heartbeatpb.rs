#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartbeatCheckRequest {
    #[prost(string, tag = "1")]
    pub service: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartbeatCheckResponse {
    #[prost(enumeration = "heartbeat_check_response::ServingStatus", tag = "1")]
    pub status: i32,
}
/// Nested message and enum types in `HeartbeatCheckResponse`.
pub mod heartbeat_check_response {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ServingStatus {
        Unknown = 0,
        Serving = 1,
        NotServing = 2,
        /// Used only by the Watch method.
        ServiceUnknown = 3,
    }
}
#[doc = r" Generated client implementations."]
pub mod heartbeat_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct HeartbeatServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl HeartbeatServiceClient<tonic::transport::Channel> {
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
    impl<T> HeartbeatServiceClient<T>
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
        #[doc = " If the requested service is unknown, the call will fail with status"]
        #[doc = " NOT_FOUND."]
        pub async fn check(
            &mut self,
            request: impl tonic::IntoRequest<super::HeartbeatCheckRequest>,
        ) -> Result<tonic::Response<super::HeartbeatCheckResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/heartbeatpb.HeartbeatService/Check");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Performs a watch for the serving status of the requested service."]
        #[doc = " The server will immediately send back a message indicating the current"]
        #[doc = " serving status.  It will then subsequently send a new message whenever"]
        #[doc = " the service's serving status changes."]
        #[doc = ""]
        #[doc = " If the requested service is unknown when the call is received, the"]
        #[doc = " server will send a message setting the serving status to"]
        #[doc = " SERVICE_UNKNOWN but will *not* terminate the call.  If at some"]
        #[doc = " future point, the serving status of the service becomes known, the"]
        #[doc = " server will send a new message with the service's serving status."]
        #[doc = ""]
        #[doc = " If the call terminates with status UNIMPLEMENTED, then clients"]
        #[doc = " should assume this method is not supported and should not retry the"]
        #[doc = " call.  If the call terminates with any other status (including OK),"]
        #[doc = " clients should retry the call with appropriate exponential backoff."]
        pub async fn watch(
            &mut self,
            request: impl tonic::IntoRequest<super::HeartbeatCheckRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::HeartbeatCheckResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/heartbeatpb.HeartbeatService/Watch");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for HeartbeatServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for HeartbeatServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "HeartbeatServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod heartbeat_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with HeartbeatServiceServer."]
    #[async_trait]
    pub trait HeartbeatService: Send + Sync + 'static {
        #[doc = " If the requested service is unknown, the call will fail with status"]
        #[doc = " NOT_FOUND."]
        async fn check(
            &self,
            request: tonic::Request<super::HeartbeatCheckRequest>,
        ) -> Result<tonic::Response<super::HeartbeatCheckResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the Watch method."]
        type WatchStream: futures_core::Stream<Item = Result<super::HeartbeatCheckResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " Performs a watch for the serving status of the requested service."]
        #[doc = " The server will immediately send back a message indicating the current"]
        #[doc = " serving status.  It will then subsequently send a new message whenever"]
        #[doc = " the service's serving status changes."]
        #[doc = ""]
        #[doc = " If the requested service is unknown when the call is received, the"]
        #[doc = " server will send a message setting the serving status to"]
        #[doc = " SERVICE_UNKNOWN but will *not* terminate the call.  If at some"]
        #[doc = " future point, the serving status of the service becomes known, the"]
        #[doc = " server will send a new message with the service's serving status."]
        #[doc = ""]
        #[doc = " If the call terminates with status UNIMPLEMENTED, then clients"]
        #[doc = " should assume this method is not supported and should not retry the"]
        #[doc = " call.  If the call terminates with any other status (including OK),"]
        #[doc = " clients should retry the call with appropriate exponential backoff."]
        async fn watch(
            &self,
            request: tonic::Request<super::HeartbeatCheckRequest>,
        ) -> Result<tonic::Response<Self::WatchStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct HeartbeatServiceServer<T: HeartbeatService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: HeartbeatService> HeartbeatServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for HeartbeatServiceServer<T>
    where
        T: HeartbeatService,
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
                "/heartbeatpb.HeartbeatService/Check" => {
                    #[allow(non_camel_case_types)]
                    struct CheckSvc<T: HeartbeatService>(pub Arc<T>);
                    impl<T: HeartbeatService>
                        tonic::server::UnaryService<super::HeartbeatCheckRequest> for CheckSvc<T>
                    {
                        type Response = super::HeartbeatCheckResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HeartbeatCheckRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).check(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CheckSvc(inner);
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
                "/heartbeatpb.HeartbeatService/Watch" => {
                    #[allow(non_camel_case_types)]
                    struct WatchSvc<T: HeartbeatService>(pub Arc<T>);
                    impl<T: HeartbeatService>
                        tonic::server::ServerStreamingService<super::HeartbeatCheckRequest>
                        for WatchSvc<T>
                    {
                        type Response = super::HeartbeatCheckResponse;
                        type ResponseStream = T::WatchStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HeartbeatCheckRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).watch(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = WatchSvc(inner);
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
    impl<T: HeartbeatService> Clone for HeartbeatServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: HeartbeatService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: HeartbeatService> tonic::transport::NamedService for HeartbeatServiceServer<T> {
        const NAME: &'static str = "heartbeatpb.HeartbeatService";
    }
}
