#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvertiseClientHeartbeatResponse {
    #[prost(enumeration = "AdvertiseClientStatus", required, tag = "1")]
    pub status: i32,
}
#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvertiseClientHeartbeatRequest {
    #[prost(string, required, tag = "1")]
    pub service: ::prost::alloc::string::String,
}
#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvertiseClientSettings {
    #[prost(string, repeated, tag = "1")]
    pub urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, required, tag = "2")]
    pub heartbeat_max_retry: u32,
    #[prost(string, required, tag = "3")]
    pub heartbeat_interval: ::prost::alloc::string::String,
    #[prost(string, required, tag = "4")]
    pub policy: ::prost::alloc::string::String,
}
#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterSettings {
    #[prost(message, required, tag = "1")]
    pub advertise_client: AdvertiseClientSettings,
    #[prost(string, repeated, tag = "2")]
    pub initial_advertise_peer_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, required, tag = "3")]
    pub initial_cluster: ::prost::alloc::string::String,
    #[prost(string, required, tag = "4")]
    pub initial_cluster_state: ::prost::alloc::string::String,
}
#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecuritySettings {
    #[prost(string, repeated, tag = "1")]
    pub cors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, required, tag = "2")]
    pub key_file: ::prost::alloc::string::String,
    #[prost(string, required, tag = "3")]
    pub cert_file: ::prost::alloc::string::String,
    #[prost(string, required, tag = "4")]
    pub peer_key_file: ::prost::alloc::string::String,
    #[prost(string, required, tag = "5")]
    pub peer_cert_file: ::prost::alloc::string::String,
}
#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogSettings {
    #[prost(string, required, tag = "1")]
    pub dir: ::prost::alloc::string::String,
    #[prost(string, required, tag = "2")]
    pub level: ::prost::alloc::string::String,
    #[prost(string, required, tag = "3")]
    pub filter: ::prost::alloc::string::String,
}
#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    #[prost(message, required, tag = "1")]
    pub cluster: ClusterSettings,
    #[prost(message, required, tag = "2")]
    pub security: SecuritySettings,
    #[prost(message, required, tag = "3")]
    pub log: LogSettings,
    #[prost(string, required, tag = "4")]
    pub config_file: ::prost::alloc::string::String,
}
#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewConfigRequest {}
#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewConfigResponse {
    #[prost(int32, required, tag = "1")]
    pub err: i32,
    #[prost(string, required, tag = "2")]
    pub msg: ::prost::alloc::string::String,
    #[prost(message, required, tag = "3")]
    pub config: Config,
}
#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGroupRequest {}
#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGroupResponse {}
#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRequest {
    #[prost(bool, required, tag = "1")]
    pub force: bool,
}
#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopResponse {
    #[prost(int32, required, tag = "1")]
    pub err: i32,
    #[prost(string, optional, tag = "2")]
    pub addr: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, required, tag = "3")]
    pub daemon_pid: i32,
    #[prost(string, required, tag = "4")]
    pub msg: ::prost::alloc::string::String,
}
#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AdvertiseClientStatus {
    Serving = 0,
    NotServing = 1,
}
#[doc = r" Generated client implementations."]
pub mod daemon_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct DaemonServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DaemonServiceClient<tonic::transport::Channel> {
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
    impl<T> DaemonServiceClient<T>
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
        pub async fn stop(
            &mut self,
            request: impl tonic::IntoRequest<super::StopRequest>,
        ) -> Result<tonic::Response<super::StopResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/daemonpb.DaemonService/Stop");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn view_config(
            &mut self,
            request: impl tonic::IntoRequest<super::ViewConfigRequest>,
        ) -> Result<tonic::Response<super::ViewConfigResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/daemonpb.DaemonService/ViewConfig");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DaemonServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DaemonServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DaemonServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod network_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct NetworkServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl NetworkServiceClient<tonic::transport::Channel> {
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
    impl<T> NetworkServiceClient<T>
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
        pub async fn view_config(
            &mut self,
            request: impl tonic::IntoRequest<super::ViewConfigRequest>,
        ) -> Result<tonic::Response<super::ViewConfigResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/daemonpb.NetworkService/ViewConfig");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn check_advertise_client(
            &mut self,
            request: impl tonic::IntoRequest<super::AdvertiseClientHeartbeatRequest>,
        ) -> Result<tonic::Response<super::AdvertiseClientHeartbeatResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/daemonpb.NetworkService/CheckAdvertiseClient",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for NetworkServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for NetworkServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "NetworkServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod daemon_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with DaemonServiceServer."]
    #[async_trait]
    pub trait DaemonService: Send + Sync + 'static {
        async fn stop(
            &self,
            request: tonic::Request<super::StopRequest>,
        ) -> Result<tonic::Response<super::StopResponse>, tonic::Status>;
        async fn view_config(
            &self,
            request: tonic::Request<super::ViewConfigRequest>,
        ) -> Result<tonic::Response<super::ViewConfigResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct DaemonServiceServer<T: DaemonService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: DaemonService> DaemonServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for DaemonServiceServer<T>
    where
        T: DaemonService,
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
                "/daemonpb.DaemonService/Stop" => {
                    #[allow(non_camel_case_types)]
                    struct StopSvc<T: DaemonService>(pub Arc<T>);
                    impl<T: DaemonService> tonic::server::UnaryService<super::StopRequest> for StopSvc<T> {
                        type Response = super::StopResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stop(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = StopSvc(inner);
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
                "/daemonpb.DaemonService/ViewConfig" => {
                    #[allow(non_camel_case_types)]
                    struct ViewConfigSvc<T: DaemonService>(pub Arc<T>);
                    impl<T: DaemonService> tonic::server::UnaryService<super::ViewConfigRequest> for ViewConfigSvc<T> {
                        type Response = super::ViewConfigResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ViewConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).view_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ViewConfigSvc(inner);
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
    impl<T: DaemonService> Clone for DaemonServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: DaemonService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DaemonService> tonic::transport::NamedService for DaemonServiceServer<T> {
        const NAME: &'static str = "daemonpb.DaemonService";
    }
}
#[doc = r" Generated server implementations."]
pub mod network_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with NetworkServiceServer."]
    #[async_trait]
    pub trait NetworkService: Send + Sync + 'static {
        async fn view_config(
            &self,
            request: tonic::Request<super::ViewConfigRequest>,
        ) -> Result<tonic::Response<super::ViewConfigResponse>, tonic::Status>;
        async fn check_advertise_client(
            &self,
            request: tonic::Request<super::AdvertiseClientHeartbeatRequest>,
        ) -> Result<tonic::Response<super::AdvertiseClientHeartbeatResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct NetworkServiceServer<T: NetworkService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: NetworkService> NetworkServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for NetworkServiceServer<T>
    where
        T: NetworkService,
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
                "/daemonpb.NetworkService/ViewConfig" => {
                    #[allow(non_camel_case_types)]
                    struct ViewConfigSvc<T: NetworkService>(pub Arc<T>);
                    impl<T: NetworkService> tonic::server::UnaryService<super::ViewConfigRequest> for ViewConfigSvc<T> {
                        type Response = super::ViewConfigResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ViewConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).view_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ViewConfigSvc(inner);
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
                "/daemonpb.NetworkService/CheckAdvertiseClient" => {
                    #[allow(non_camel_case_types)]
                    struct CheckAdvertiseClientSvc<T: NetworkService>(pub Arc<T>);
                    impl<T: NetworkService>
                        tonic::server::UnaryService<super::AdvertiseClientHeartbeatRequest>
                        for CheckAdvertiseClientSvc<T>
                    {
                        type Response = super::AdvertiseClientHeartbeatResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AdvertiseClientHeartbeatRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).check_advertise_client(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CheckAdvertiseClientSvc(inner);
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
    impl<T: NetworkService> Clone for NetworkServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: NetworkService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: NetworkService> tonic::transport::NamedService for NetworkServiceServer<T> {
        const NAME: &'static str = "daemonpb.NetworkService";
    }
}
