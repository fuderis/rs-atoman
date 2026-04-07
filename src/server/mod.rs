pub mod response;
pub use response::Response;

use crate::prelude::*;
use axum::{
    Router,
    handler::Handler,
    routing::{get, post},
};
use std::net::SocketAddr;
use tokio::net::TcpListener;

/// The Axum server wrapper
pub struct Server {
    router: Router,
}

impl Server {
    /// Creates a new Axum server
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

    /// Add any route (universal)
    pub fn route(mut self, path: &str, method_router: axum::routing::MethodRouter) -> Self {
        self.router = self.router.route(path, method_router);
        self
    }

    /// Add the POST-page handler
    pub fn post<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, ()>,
        T: 'static,
    {
        self.router = self.router.route(path, post(handler));
        self
    }

    /// Add the GET-page handler
    pub fn get<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, ()>,
        T: 'static,
    {
        self.router = self.router.route(path, get(handler));
        self
    }

    /// Launching the server at a specific address
    pub async fn run(self, addr: impl Into<SocketAddr>) -> Result<()> {
        let listener = TcpListener::bind(addr.into()).await?;
        axum::serve(listener, self.router).await?;
        Ok(())
    }
}
