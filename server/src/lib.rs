#![forbid(unsafe_code)]

use std::sync::Arc;

use anyhow::{Context as _, Result};
use axum::{Router, routing::get};
use dill::{Catalog, CatalogBuilder};

#[derive(Debug)]
struct StatusService;

impl StatusService {
    fn health(&self) -> &'static str {
        "ok"
    }
}

pub fn register(builder: &mut CatalogBuilder) {
    builder.add_value(StatusService);
}

pub fn router(catalog: &Catalog) -> Result<Router> {
    let service = catalog
        .get_one::<StatusService>()
        .context("解析插件状态服务失败")?;
    Ok(Router::new().route(
        "/api/plugins/aio-plugin-hello/health",
        get(move || {
            let service = Arc::clone(&service);
            async move { service.health() }
        }),
    ))
}
