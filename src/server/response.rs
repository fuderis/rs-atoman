use axum::{
    body::Body,
    http::{HeaderName, HeaderValue, StatusCode, header, response::Builder},
    response::{IntoResponse, Response as AxumResponse},
};
use bytes::Bytes;
use futures::Stream as FuturesStream;

/// The HTTP response builder
pub struct Response {
    builder: Builder,
    body: Body,
}

impl Response {
    // --- HTTP STATUS ---

    /// Creates a new response from code
    pub fn code(code: u16) -> Self {
        Self {
            builder: Builder::new().status(code),
            body: Body::empty(),
        }
    }

    /// Creates a new response with 200 code
    pub fn ok() -> Self {
        Self::code(200)
    }

    /// Creates a new response with 201 code
    pub fn created() -> Self {
        Self::code(201)
    }

    /// Creates a new response with 204 code
    pub fn no_content() -> Self {
        Self::code(204)
    }

    /// Creates a new response with 301 code
    pub fn redirect() -> Self {
        Self::code(301)
    }

    /// Creates a new response with 302 code
    pub fn temp_redirect() -> Self {
        Self::code(302)
    }

    /// Creates a new response with 304 code
    pub fn not_modified() -> Self {
        Self::code(304)
    }

    /// Creates a new response with 400 code
    pub fn bad_request() -> Self {
        Self::code(400)
    }

    /// Creates a new response with 401 code
    pub fn unauthorized() -> Self {
        Self::code(401)
    }

    /// Creates a new response with 403 code
    pub fn forbidden() -> Self {
        Self::code(403)
    }

    /// Creates a new response with 404 code
    pub fn not_found() -> Self {
        Self::code(404)
    }

    /// Creates a new response with 405 code
    pub fn bad_method() -> Self {
        Self::code(405)
    }

    /// Creates a new response with 409 code
    pub fn conflict() -> Self {
        Self::code(409)
    }

    /// Creates a new response with 413 code
    pub fn too_large() -> Self {
        Self::code(413)
    }

    /// Creates a new response with 422 code
    pub fn bad_entity() -> Self {
        Self::code(422)
    }

    /// Creates a new response with 429 code
    pub fn rate_limited() -> Self {
        Self::code(429)
    }

    /// Creates a new response with 500 code
    pub fn server_error() -> Self {
        Self::code(500)
    }

    /// Creates a new response with 502 code
    pub fn bad_gateway() -> Self {
        Self::code(502)
    }

    /// Creates a new response with 503 code
    pub fn unavailable() -> Self {
        Self::code(503)
    }

    /// Creates a new response with 504 code
    pub fn timeout() -> Self {
        Self::code(504)
    }

    /// Changes the status code (for example, 201, 404, 500)
    pub fn status(mut self, status: impl Into<StatusCode>) -> Self {
        self.builder = self.builder.status(status.into());
        self
    }

    // --- HTTP HEADER ---

    /// Sets the header
    pub fn header(mut self, key: impl Into<HeaderName>, value: impl Into<HeaderValue>) -> Self {
        self.builder = self.builder.header(key, value);
        self
    }

    /// Sets the HTTPS-only connect header
    pub fn https_only(self, seconds: u64) -> Self {
        self.header(
            header::STRICT_TRANSPORT_SECURITY,
            format!("max-age={}; includeSubDomains", seconds)
                .parse::<HeaderValue>()
                .unwrap(),
        )
    }

    /// Sets the iframe options header
    pub fn no_iframe(self) -> Self {
        self.header(
            header::X_FRAME_OPTIONS,
            "DENY".parse::<HeaderValue>().unwrap(),
        )
    }

    /// Sets the content sniff options header
    pub fn no_sniff(self) -> Self {
        self.header(
            header::X_CONTENT_TYPE_OPTIONS,
            "nosniff".parse::<HeaderValue>().unwrap(),
        )
    }

    /// Sets the cache control options header
    pub fn cache_control(self, value: impl Into<HeaderValue>) -> Self {
        self.header(header::CACHE_CONTROL, value)
    }

    /// Sets the cache control options header
    pub fn no_cache(self) -> Self {
        self.cache_control(
            "no-store, no-cache, must-revalidate"
                .parse::<HeaderValue>()
                .unwrap(),
        )
    }

    /// Sets the content-type header
    pub fn content_type(self, value: impl Into<HeaderValue>) -> Self {
        self.header(header::CONTENT_TYPE, value)
    }

    /// Sets the content-type=TEXT header
    pub fn content_type_text(self) -> Self {
        self.content_type("text/plain; charset=utf-8".parse::<HeaderValue>().unwrap())
    }

    /// Sets the content-type=JSON header
    pub fn content_type_json(self) -> Self {
        self.content_type("application/json".parse::<HeaderValue>().unwrap())
    }

    /// Sets the content-type=HTML header
    pub fn content_type_html(self) -> Self {
        self.content_type("text/html; charset=utf-8".parse::<HeaderValue>().unwrap())
    }

    /// Sets the content-type=EVENT-STREAM header
    pub fn content_type_stream(self) -> Self {
        self.header(
            header::CONTENT_TYPE,
            "text/event-stream".parse::<HeaderValue>().unwrap(),
        )
        .header(
            header::CACHE_CONTROL,
            "no-cache".parse::<HeaderValue>().unwrap(),
        )
        .header(
            header::CONNECTION,
            "keep-alive".parse::<HeaderValue>().unwrap(),
        )
        .header(
            header::X_CONTENT_TYPE_OPTIONS,
            "nosniff".parse::<HeaderValue>().unwrap(),
        )
    }

    /// Sets the access control header
    pub fn allow_origin(self) -> Self {
        self.header(
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            "".parse::<HeaderValue>().unwrap(),
        )
    }

    /// Sets the access control header
    pub fn allow_methods(self) -> Self {
        self.header(
            header::ACCESS_CONTROL_ALLOW_METHODS,
            "".parse::<HeaderValue>().unwrap(),
        )
    }

    /// Sets the access control header
    pub fn allow_headers(self) -> Self {
        self.header(
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            "".parse::<HeaderValue>().unwrap(),
        )
    }

    /// Sets the file attachment header
    pub fn attachment(self, filename: &str) -> Self {
        self.header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename)
                .parse::<HeaderValue>()
                .unwrap(),
        )
    }

    /// Sets the redirect location header
    pub fn location(self, uri: &'static str) -> Self {
        self.header(header::LOCATION, uri.parse::<HeaderValue>().unwrap())
    }

    // --- HTTP BODY ---

    /// Sets the body (string, bytes, or stream)
    pub fn body(mut self, body: impl Into<Body>) -> Self {
        self.body = body.into();
        self
    }

    /// Sets the plain text body (forced UTF-8)
    pub fn text(self, text: impl Into<String>) -> Self {
        self.content_type_text().body(text.into())
    }

    /// Sets the HTML content body
    pub fn html(mut self, html: impl Into<String>) -> Self {
        self = self.content_type_html();
        self.body = Body::from(html.into());
        self
    }

    /// Sets the JSON content body
    pub fn json<T: serde::Serialize>(mut self, v: &T) -> Self {
        let bytes = serde_json::to_vec(v).unwrap_or_default();
        self = self.content_type_json();
        self.body(bytes)
    }

    /// Sets the stream event body
    pub fn stream<S, E>(mut self, stream: S) -> Self
    where
        S: FuturesStream<Item = Result<Bytes, E>> + Send + 'static,
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        self = self.content_type_stream();
        self.body = Body::from_stream(stream);
        self
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> AxumResponse {
        self.builder.body(self.body).unwrap_or_else(|_| {
            // if builder is broken (unlikely), return 500
            AxumResponse::builder()
                .status(500)
                .body(Body::from("Internal Server Error"))
                .unwrap()
        })
    }
}
