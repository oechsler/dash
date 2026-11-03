#[cfg(feature = "postgres")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let bind = std::env::var("DASH_HTTP_BIND").unwrap_or_else(|_| "127.0.0.1:3000".to_owned());
    let listener = tokio::net::TcpListener::bind(&bind).await?;
    let api = dash::composition::PostgresConnector::http_api_from_env().await?;
    axum::serve(listener, api.router()).await?;
    Ok(())
}

#[cfg(not(feature = "postgres"))]
fn main() {
    eprintln!("The HTTP server requires the `postgres` feature.");
}
