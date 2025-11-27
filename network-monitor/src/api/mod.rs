// src/api/mod.rs
pub mod handlers;
pub mod routes;

use actix_web::{
    dev::{ServiceRequest, ServiceResponse, Transform},
    Error, Result,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::task::{Context, Poll};
use actix_web::body::{MessageBody, BoxBody};


// 实现统一错误处理中间件
pub struct ErrorHandlingMiddleware;

impl<S, B> Transform<S, ServiceRequest> for ErrorHandlingMiddleware
where
    S: actix_service::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = ErrorHandlingMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ErrorHandlingMiddlewareService { service }))
    }
}

pub struct ErrorHandlingMiddlewareService<S> {
    service: S,
}

impl<S, B> actix_service::Service<ServiceRequest> for ErrorHandlingMiddlewareService<S>
where
    S: actix_service::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await;
            match res {
                Ok(response) => Ok(response.map_into_boxed_body()),
                Err(error) => {
                    eprintln!("API Error: {}", error);
                    // 在移动req之前先提取所需信息
                    Err(error)
                }
            }
        })
    }
}