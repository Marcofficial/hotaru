use bytes::Bytes;
use hyper::{HeaderMap, Method, Request, Uri, Version};

/// Fully buffered Hyper request exposed to Hotaru endpoints.
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

    pub fn path_segments(&self) -> impl Iterator<Item = &str> {
        self.path().split('/').filter(|segment| !segment.is_empty())
    }

    pub fn query(&self) -> Option<&str> {
        self.inner.uri().query()
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
