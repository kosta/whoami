use std::net::SocketAddr;

use maud::{html, Markup};
use axum::{extract::ConnectInfo, http::HeaderMap, routing::get, Router};

async fn hello_world(ConnectInfo(addr): ConnectInfo<SocketAddr>, headers: HeaderMap) -> Markup {
    html! {
        h1 { "Your IP" }
        p { "Your IP: " (addr.ip()) }
        p { "You connected from port: " (addr.port()) }
        h1 { "Request Headers" }
        p { "You sent the following headers:" }
        ul {
            @for (name, value) in headers {
                li {
                    @if let Some(name) = name {
                        @if let Ok(value) = value.to_str() {
                            (name) ": " (value)
                        }
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/whoami", get(hello_world));

    // run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
