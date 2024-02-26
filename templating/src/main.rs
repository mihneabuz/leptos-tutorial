use leptos::view;

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/home", get(home))
        .route("/leptos", get(leptos))
        .route("/leptos_explicit", get(leptos_explicit));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn home() -> impl IntoResponse {
    Html::from("<div><h1>Hello, World!</h1></div>")
}

async fn leptos_explicit() -> impl IntoResponse {
    let html = leptos::ssr::render_to_string(|| {
        view! {
          <div>
            <h2>"Hello Leptos!"</h2>
            <h4>"view macro rocks!"</h4>
          </div>
        }
    });

    Html::from(html.to_string())
}

macro_rules! leptos_html {
    ($($x:tt)*) => {
        Html::from(leptos::ssr::render_to_string(|| view! { $($x)* }).to_string())
    };
}

async fn leptos() -> impl IntoResponse {
    leptos_html! (
      <div>
        <h2>"Hello Leptos!"</h2>
        <h4>"view macro rocks!"</h4>
      </div>
    )
}
