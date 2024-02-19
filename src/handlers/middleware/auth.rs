use actix_web::{
    body::BoxBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures::{
    future::{ready, Ready},
    Future,
};
use std::{pin::Pin, task::Poll};
use uuid::Uuid;

use crate::{consts::headers, utils::jwt::get_claims_from_token};

pub struct UserID(Uuid);

impl UserID {
    pub fn new(id: Uuid) -> Self {
        UserID(id)
    }
}

impl Into<Uuid> for &UserID {
    fn into(self) -> Uuid {
        self.0
    }
}

pub struct AuthMiddleware;

impl<S> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;

    type Error = Error;

    type Transform = AuthMiddlewareService<S>;

    type InitError = ();

    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;

    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get(headers::AUTHORIZATION);

        let fut = if let Some(token) = auth_header {
            let token = match token.to_str() {
                Ok(token) => token.to_string(),
                Err(e) => {
                    tracing::error!("failed to convert token to string: {:?}", e);
                    return return_unauthorized(req);
                }
            };

            match get_claims_from_token(token) {
                Ok(claims) => {
                    let subject = claims.sub;
                    match Uuid::parse_str(&subject) {
                        Ok(id) => {
                            req.extensions_mut().insert(UserID::new(id));
                            Box::pin(self.service.call(req)) as Self::Future
                        }
                        Err(e) => {
                            tracing::error!("token did not contain valid subject id: {:?}", e);
                            return_unauthorized(req)
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("failed to get claims from token: {:?}", e);
                    return_unauthorized(req)
                }
            }
        } else {
            return_unauthorized(req)
        };

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

fn return_unauthorized(
    req: ServiceRequest,
) -> Pin<Box<dyn Future<Output = Result<ServiceResponse<BoxBody>, Error>>>> {
    let body: Result<ServiceResponse<BoxBody>, Error> =
        Ok(req.into_response(HttpResponse::Unauthorized().finish()));
    Box::pin(async move { body })
}
