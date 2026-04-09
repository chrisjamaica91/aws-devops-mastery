use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// Response structures (automatically serialized to JSON)
#[derive(Serialize)]
struct HelloResponse {
    message: String,
    service: String,
    version: String,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

#[derive(Serialize)]
struct StatusResponse {
    status: String,
    uptime_ms: u128,
    rust_version: String,
}

#[derive(Serialize, Deserialize)]
struct ProcessRequest {
    data: Vec<i32>,
}

#[derive(Serialize)]
struct ProcessResponse {
    original: Vec<i32>,
    sum: i32,
    average: f64,
    min: i32,
    max: i32,
    processed_count: usize,
}

// Store startup time globally
static mut START_TIME: Option<SystemTime> = None;

// Health endpoints (for Kubernetes probes)
#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
    })
}

#[get("/ready")]
async fn ready() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ready".to_string(),
    })
}

// API endpoints
#[get("/api/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(HelloResponse {
        message: "Hello from Rust Processor!".to_string(),
        service: "rust-processor".to_string(),
        version: "1.0.0".to_string(),
    })
}

#[get("/api/status")]
async fn status() -> impl Responder {
    let uptime = unsafe {
        START_TIME
            .map(|t| SystemTime::now().duration_since(t).unwrap().as_millis())
            .unwrap_or(0)
    };

    HttpResponse::Ok().json(StatusResponse {
        status: "running".to_string(),
        uptime_ms: uptime,
        rust_version: rustc_version().to_string(),
    })
}

// Data processing endpoint (demonstrates Rust's computational efficiency)
#[actix_web::post("/api/process")]
async fn process(data: web::Json<ProcessRequest>) -> impl Responder {
    let numbers = &data.data;
    
    if numbers.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Data array cannot be empty"
        }));
    }

    let sum: i32 = numbers.iter().sum();
    let count = numbers.len();
    let average = sum as f64 / count as f64;
    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();

    HttpResponse::Ok().json(ProcessResponse {
        original: numbers.clone(),
        sum,
        average,
        min,
        max,
        processed_count: count,
    })
}

fn rustc_version() -> &'static str {
    env!("CARGO_PKG_RUST_VERSION")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // Record start time
    unsafe {
        START_TIME = Some(SystemTime::now());
    }

    log::info!("Starting Rust Processor v1.0.0");
    log::info!("Binding to 0.0.0.0:3000");

    HttpServer::new(|| {
        App::new()
            .service(health)
            .service(ready)
            .service(hello)
            .service(status)
            .service(process)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}