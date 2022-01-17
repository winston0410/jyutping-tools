use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, LocalBoxFuture, Ready};
use futures::Future;
//NOTE Dependency for the middleware's content, should be extracted later
use actix_web::http::header::{
    CacheControl, CacheDirective, Expires, HeaderValue, CACHE_CONTROL, EXPIRES,
};
use std::time::{Duration, SystemTime};

pub struct CacheHeader {
    max_age: u32,
}

impl Default for CacheHeader {
    fn default() -> CacheHeader {
        CacheHeader { max_age: 31536000 }
    }
}

impl<S, B> Transform<S, ServiceRequest> for CacheHeader
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CacheHeaderMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CacheHeaderMiddleware {
            service,
            max_age: self.max_age,
        })
    }
}

pub struct CacheHeaderMiddleware<S> {
    service: S,
    max_age: u32,
}

impl<S, B> Service<ServiceRequest> for CacheHeaderMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_service::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);
        let max_age = self.max_age.to_owned();

        Box::pin(async move {
            let mut res = fut.await?;

            let headers = res.headers_mut();
            let cache_value = CacheControl(vec![
                CacheDirective::Public,
                CacheDirective::MaxAge(max_age),
                //TODO Contribute this code to actix web
                // CacheDirective::Immutable
            ])
            .to_string();

            let expiration_date = SystemTime::now() + Duration::from_secs(max_age.into());
            let expires_value = Expires(expiration_date.into()).to_string();
            headers.insert(CACHE_CONTROL, HeaderValue::try_from(cache_value)?);
            headers.insert(EXPIRES, HeaderValue::try_from(expires_value)?);

            Ok(res)
        })
    }
}
