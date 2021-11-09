#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RlUserId {
    #[prost(int64, tag = "1")]
    pub rl_user_id: i64,
    #[prost(string, tag = "2")]
    pub time: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchHistoryReply {
    #[prost(int32, tag = "1")]
    pub count: i32,
    #[prost(message, repeated, tag = "2")]
    pub matches: ::prost::alloc::vec::Vec<MatchHistoryEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchHistoryEntry {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub time: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub elo: i32,
    #[prost(int32, tag = "4")]
    pub elo_rating: i32,
    #[prost(int32, tag = "5")]
    pub rank: i32,
    #[prost(int32, tag = "6")]
    pub wins: i32,
    #[prost(int32, tag = "7")]
    pub losses: i32,
    #[prost(int32, tag = "8")]
    pub win_streak: i32,
    #[prost(string, tag = "9")]
    pub match_type: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub team_size: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub versus: ::prost::alloc::string::String,
}
#[doc = r" Generated server implementations."]
pub mod player_page_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with PlayerPageServer."]
    #[async_trait]
    pub trait PlayerPage: Send + Sync + 'static {
        async fn get_player_history_matches(
            &self,
            request: tonic::Request<super::RlUserId>,
        ) -> Result<tonic::Response<super::MatchHistoryReply>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct PlayerPageServer<T: PlayerPage> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PlayerPage> PlayerPageServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PlayerPageServer<T>
    where
        T: PlayerPage,
        B: Body + Send + 'static,
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
                "/player.PlayerPage/GetPlayerHistoryMatches" => {
                    #[allow(non_camel_case_types)]
                    struct GetPlayerHistoryMatchesSvc<T: PlayerPage>(pub Arc<T>);
                    impl<T: PlayerPage> tonic::server::UnaryService<super::RlUserId> for GetPlayerHistoryMatchesSvc<T> {
                        type Response = super::MatchHistoryReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RlUserId>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_player_history_matches(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPlayerHistoryMatchesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
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
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: PlayerPage> Clone for PlayerPageServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: PlayerPage> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PlayerPage> tonic::transport::NamedService for PlayerPageServer<T> {
        const NAME: &'static str = "player.PlayerPage";
    }
}
