use bytes::Bytes;
use http_body_util::Full;
use hyper::{HeaderMap, Method, Request, Response, StatusCode, Uri, Version};

/// Fully buffered Hyper request exposed to Hotaru handlers.
#[derive(Debug)]
pub struct HyperRequest {
    inner: Request<Bytes>,
}

impl HyperRequest {
    pub(crate) fn new(inner: Request<Bytes>) -> Self {
        Self { inner }
    }

    pub fn method(&self) -> &Method {
        self.inner.method()
    }

    pub fn uri(&self) -> &Uri {
        self.inner.uri()
    }

    pub fn path(&self) -> &str {
        self.inner.uri().path()
    }

    pub fn version(&self) -> Version {
        self.inner.version()
    }

    pub fn headers(&self) -> &HeaderMap {
        self.inner.headers()
    }

    pub fn body(&self) -> &Bytes {
        self.inner.body()
    }

    pub fn as_inner(&self) -> &Request<Bytes> {
        &self.inner
    }

    pub fn as_inner_mut(&mut self) -> &mut Request<Bytes> {
        &mut self.inner
    }

    pub fn into_inner(self) -> Request<Bytes> {
        self.inner
    }
}

impl Default for HyperRequest {
    fn default() -> Self {
        Self {
            inner: Request::new(Bytes::new()),
        }
    }
}

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
        Self::new(body.into())
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
