use std::future::{ready, Ready};
use actix_session::{CookieSession, Session};

use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http, Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;

// middleware 1
pub struct Auth;

// implement Transform
impl<S, B> Transform<S, ServiceRequest> for Auth
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}
pub struct AuthMiddleware<S> {
    service: S,
}

// implement Service
impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        // Change this to see the change in outcome in the browser.
        // Usually this boolean would be acquired from a password check or other auth verification.
        let is_logged_in = false;
        // TODO something
        // Don't forward to `/login` if we are already on `/login`.
        if !is_logged_in && request.path() != "/login" {
            let (request, _pl) = request.into_parts();

            let response = HttpResponse::Found()
                .insert_header((http::header::LOCATION, "/login"))
                .finish()
                // constructed responses map to "right" body
                .map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        let res = self.service.call(request);

        Box::pin(async move {
            // forwarded responses map to "left" body
            res.await.map(ServiceResponse::map_into_left_body)
        })
    }
}
// middleware 1 end

// middleware 2
pub struct BaseAuth;

// implement Transform
impl<S, B> Transform<S, ServiceRequest> for BaseAuth
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = BaseAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(BaseAuthMiddleware { service }))
    }
}

pub struct BaseAuthMiddleware<S> {
    service: S,
}

// implement Service
impl<S, B> Service<ServiceRequest> for BaseAuthMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        // Change this to see the change in outcome in the browser.
        // Usually this boolean would be acquired from a password check or other auth verification.
        let is_logged_in = false;
        // TODO something
        // Don't forward to `/login` if we are already on `/login`.
        if !is_logged_in && request.path() != "/login" {
            let (request, _pl) = request.into_parts();

            let response = HttpResponse::Found()
                .insert_header((http::header::LOCATION, "/login"))
                .finish()
                // constructed responses map to "right" body
                .map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        let res = self.service.call(request);

        Box::pin(async move {
            // forwarded responses map to "left" body
            res.await.map(ServiceResponse::map_into_left_body)
        })
    }
}



// middleware request
pub struct Request;

// implement Transform
impl<S, B> Transform<S, ServiceRequest> for Request
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = RequestMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestMiddleware { service }))
    }
}

pub struct RequestMiddleware<S> {
    service: S,
}

// implement Service
impl<S, B> Service<ServiceRequest> for RequestMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, mut request: ServiceRequest) -> Self::Future {
        // Change this to see the change in outcome in the browser.
        // Usually this boolean would be acquired from a password check or other auth verification.
        // TODO something
        println!("ssssssssssssss");
        if false {
            let (request, _pl) = request.into_parts();
            let response = HttpResponse::Found()
                .insert_header((http::header::LOCATION, "/login"))
                .finish()
                // constructed responses map to "right" body
                .map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        let res = self.service.call(request);
        Box::pin(async move {
            // forwarded responses map to "left" body
            res.await.map(ServiceResponse::map_into_left_body)
        })
    }
}