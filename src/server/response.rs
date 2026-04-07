use axum::{
    body::Body,
    http::{HeaderValue, StatusCode, header, response::Builder},
    response::{IntoResponse, Response as AxumResponse},
};
use bytes::Bytes;
use futures::Stream as FuturesStream;

pub struct Response {
    builder: Builder,
    body: Body,
}

impl Response {
    /// Создает пустой успешный ответ
    pub fn ok() -> Self {
        Self {
            builder: Builder::new().status(200),
            body: Body::empty(),
        }
    }

    /// Установить статус (например, 201, 404, 500)
    pub fn status(mut self, status: impl Into<StatusCode>) -> Self {
        self.builder = self.builder.status(status.into());
        self
    }

    /// Добавить заголовок
    pub fn header(mut self, key: &'static str, value: impl Into<HeaderValue>) -> Self {
        self.builder = self.builder.header(key, value.into());
        self
    }

    /// Установить тело (строка, байты, или твой стрим)
    pub fn body(mut self, body: impl Into<Body>) -> Self {
        self.body = body.into();
        self
    }

    // --- BODY METHODS ---

    /// Отправка обычного текста с принудительным UTF-8
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self = self.content_type_text();
        self.body = Body::from(text.into());
        self
    }

    /// Отправка HTML разметки
    pub fn html(mut self, html: impl Into<String>) -> Self {
        self = self.content_type_html();
        self.body = Body::from(html.into());
        self
    }

    /// Хелпер для быстрой отправки JSON
    pub fn json<T: serde::Serialize>(mut self, v: &T) -> Self {
        let bytes = serde_json::to_vec(v).unwrap_or_default();

        self = self.content_type_text();
        self.body(bytes)
    }

    /// Удобный метод для стриминга (алиас для body)
    pub fn stream<S, E>(mut self, stream: S) -> Self
    where
        S: FuturesStream<Item = Result<Bytes, E>> + Send + 'static,
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        self = self.content_type_stream();
        self.body = Body::from_stream(stream);
        self
    }

    // -- HEADER METHODS --

    pub fn content_type(mut self, value: &'static str) -> Self {
        self.builder = self.builder.header(header::CONTENT_TYPE, value);
        self
    }

    pub fn content_type_json(self) -> Self {
        self.content_type("application/json")
    }

    pub fn content_type_text(self) -> Self {
        self.content_type("text/plain; charset=utf-8")
    }

    pub fn content_type_html(self) -> Self {
        self.content_type("text/html; charset=utf-8")
    }

    pub fn content_type_stream(mut self) -> Self {
        self.builder = self
            .builder
            .header(header::CONTENT_TYPE, "text/event-stream")
            .header(header::CACHE_CONTROL, "no-cache")
            .header(header::CONNECTION, "keep-alive")
            .header("x-content-type-options", "nosniff");
        self
    }
}

/// Магия: Axum сам вызовет этот метод в конце твоего обработчика
impl IntoResponse for Response {
    fn into_response(self) -> AxumResponse {
        self.builder.body(self.body).unwrap_or_else(|_| {
            // Если билдер сломался (маловероятно), отдаем 500
            AxumResponse::builder()
                .status(500)
                .body(Body::from("Internal Server Error"))
                .unwrap()
        })
    }
}
