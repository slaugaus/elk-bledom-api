use crate::{
	hbs::register_templates,
	types::{Config, HbsEngine, Page},
};
use axum::{
	Router,
	body::Bytes,
	http::StatusCode,
	response::IntoResponse,
	routing::{get, post},
	serve,
};
use axum_template::{RenderHtml, TemplateEngine, engine::Engine};
use handlebars::Handlebars;
use tokio::net::TcpListener;

mod hbs;
mod types;

#[tokio::main]
async fn main() {
	let mut hbs = Handlebars::new();
	register_templates(&mut hbs).unwrap();

	let app = Router::new()
		.route("/", get(ui))
		.route("/api/echo", post(echo))
		.route("/api/ping", get(|| async { "Pong" }))
		.with_state(Engine::from(hbs));

	let listener = TcpListener::bind("0.0.0.0:18080").await.unwrap();

	println!("Serving on {}", listener.local_addr().unwrap());
	serve(listener, app).await.unwrap();
}

async fn ui(engine: HbsEngine) -> impl IntoResponse {
	// TODO: config live load
	let result: Result<Config, yaml_serde::Error> =
		yaml_serde::from_str(include_str!("../config.yml"));

	let content = match result {
		Ok(config) => {
			println!("Loaded {config:?}");
			engine
				.render("ui", config)
				.unwrap_or_else(|err| format!("{}", err))
		}

		Err(err) => format!("{}", err),
	};

	RenderHtml("layout", engine, Page { content })
}

async fn echo(body: Bytes) -> Result<String, StatusCode> {
	if let Ok(string) = String::from_utf8(body.to_vec()) {
		Ok(string)
	} else {
		Err(StatusCode::BAD_REQUEST)
	}
}
