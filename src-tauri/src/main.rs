#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use opentelemetry::{global, sdk::trace::Tracer};
use std::error::Error;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
#[allow(clippy::expect_used)]
fn main() {
    init_tracer();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init_tracer() {
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("mini-redis")
        .install_simple()
        .unwrap();
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    tracing_subscriber::registry()
        .with(opentelemetry)
        .with(fmt::Layer::default())
        .try_init()
        .unwrap();

    // Ok(tracer)
}

#[tauri::command]
fn greet(name: &str) -> String {
    tracing::info!("greet command was called");
    format!("Hello, {name}!")
}
