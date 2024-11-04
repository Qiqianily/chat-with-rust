use std::ops::Deref;
use std::sync::Arc;
use axum::Router;
use axum::routing::{get, patch, post};

mod config;
mod handlers;

pub use config::AppConfig;
use handlers::*;

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    inner: Arc<AppStateInner>,
}
#[allow(unused)]
#[derive(Debug)]
pub(crate) struct AppStateInner {
    pub(crate) config: AppConfig,
}
// 实现解引用 trait
impl Deref for AppState{
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
//  实现构造函数
impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            inner: Arc::new(AppStateInner{config}),
        }
    }
}

// 定义路由
pub fn get_router(config: AppConfig) -> Router {
    let state = AppState::new(config);
    let api = Router::new()
        .route("/signin", post(signin_handler))
        .route("/signup", post(signup_handler))
        .route("/chat", get(list_chat_handler).post(create_chat_handler))
        .route("/chat/:id", patch(update_chat_handler).delete(delete_chat_handler).post(send_message_handler))
        .route("/chat/:id/messages", get(list_message_handler));
    // 返回路由

    Router::new()
        .route("/", get(index_handler))
        .nest("/api", api)
        .with_state(state)
}