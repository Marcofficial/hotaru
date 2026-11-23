use bytes::Bytes;
use http_body_util::Full;
use hyper::header::{CONTENT_TYPE, HeaderValue};
use hyper::{HeaderMap, Response, StatusCode};

/// Hyper response produced by a Hotaru endpoint.
pub struct HyperResponse {
    inner: Response<Full<Bytes>>,
}

impl HyperResponse {
    pub fn new(body: impl Into<Bytes>) -> Self {
        Self {
            inner: Response::new(Full::new(body.into())),
        }
    }

    pub fn text(body: impl Into<String>) -> Self {
        let mut response = Self::new(body.into());
        response.inner.headers_mut().insert(
            CONTENT_TYPE,
            HeaderValue::from_static("text/plain; charset=utf-8"),
        );
        response
    }

    pub fn html(body: impl Into<Bytes>) -> Self {
        let mut response = Self::new(body);
        response.inner.headers_mut().insert(
            CONTENT_TYPE,
            HeaderValue::from_static("text/html; charset=utf-8"),
        );
        response
    }

    pub fn status(mut self, status: StatusCode) -> Self {
        *self.inner.status_mut() = status;
        self
    }

    pub fn headers(&self) -> &HeaderMap {
        self.inner.headers()
    }

    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        self.inner.headers_mut()
    }

    pub fn as_inner(&self) -> &Response<Full<Bytes>> {
        &self.inner
    }

    pub fn as_inner_mut(&mut self) -> &mut Response<Full<Bytes>> {
        &mut self.inner
    }

    pub fn into_inner(self) -> Response<Full<Bytes>> {
        self.inner
    }
}

impl Default for HyperResponse {
    fn default() -> Self {
        Self::new(Bytes::new())
    }
}
