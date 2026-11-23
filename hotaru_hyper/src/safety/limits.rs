use core::time::Duration;

const DEFAULT_MAX_BODY_SIZE: usize = 2 * 1024 * 1024;
const DEFAULT_MAX_HEADERS: usize = 100;
const DEFAULT_MAX_HEADER_SIZE: usize = 32 * 1024;
const DEFAULT_HEADER_READ_TIMEOUT: Duration = Duration::from_secs(30);
const DEFAULT_KEEP_ALIVE: bool = true;

/// Limits and connection behaviour for the Hyper-backed protocols.
///
/// Each field tracks whether it was explicitly set so route-level values can
/// overlay the protocol baseline without resetting unrelated limits.
#[derive(Debug, Clone)]
pub struct HyperSafety {
    max_body_size: Option<usize>,
    max_headers: Option<usize>,
    max_header_size: Option<usize>,
    header_read_timeout: Option<Duration>,
    keep_alive: Option<bool>,
}

impl HyperSafety {
    pub fn new() -> Self {
        Self {
            max_body_size: None,
            max_headers: None,
            max_header_size: None,
            header_read_timeout: None,
            keep_alive: None,
        }
    }

    pub fn max_body_size(&self) -> usize {
        self.max_body_size.unwrap_or(DEFAULT_MAX_BODY_SIZE)
    }

    pub fn max_headers(&self) -> usize {
        self.max_headers.unwrap_or(DEFAULT_MAX_HEADERS)
    }

    pub fn max_header_size(&self) -> usize {
        self.max_header_size.unwrap_or(DEFAULT_MAX_HEADER_SIZE)
    }

    pub fn header_read_timeout(&self) -> Duration {
        self.header_read_timeout
            .unwrap_or(DEFAULT_HEADER_READ_TIMEOUT)
    }

    pub fn keep_alive(&self) -> bool {
        self.keep_alive.unwrap_or(DEFAULT_KEEP_ALIVE)
    }

    pub fn with_max_body_size(mut self, value: usize) -> Self {
        self.max_body_size = Some(value);
        self
    }

    pub fn with_max_headers(mut self, value: usize) -> Self {
        self.max_headers = Some(value);
        self
    }

    pub fn with_max_header_size(mut self, value: usize) -> Self {
        self.max_header_size = Some(value);
        self
    }

    pub fn with_header_read_timeout(mut self, value: Duration) -> Self {
        self.header_read_timeout = Some(value);
        self
    }

    pub fn with_keep_alive(mut self, value: bool) -> Self {
        self.keep_alive = Some(value);
        self
    }

    /// Overlays only explicitly configured fields from `source`.
    pub fn update(&mut self, source: &Self) {
        if source.max_body_size.is_some() {
            self.max_body_size = source.max_body_size;
        }
        if source.max_headers.is_some() {
            self.max_headers = source.max_headers;
        }
        if source.max_header_size.is_some() {
            self.max_header_size = source.max_header_size;
        }
        if source.header_read_timeout.is_some() {
            self.header_read_timeout = source.header_read_timeout;
        }
        if source.keep_alive.is_some() {
            self.keep_alive = source.keep_alive;
        }
    }
}

impl Default for HyperSafety {
    fn default() -> Self {
        Self::new()
    }
}

/// Backward-compatible name retained for the prototype API.
pub type HyperHttpConfig = HyperSafety;
