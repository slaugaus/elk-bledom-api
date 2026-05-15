use axum::{Router, routing::get, serve};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
	let static_files = ServeDir::new("./dist");
	let app = Router::new()
		.route(
			"/api/health",
			get(|| async { "it's fine. i'm fine. everything's fine." }),
		)
		.fallback_service(static_files);

	let listener = TcpListener::bind("0.0.0.0:18080").await.unwrap();

	println!("Serving on {}", listener.local_addr().unwrap());
	serve(listener, app).await.unwrap();
}
