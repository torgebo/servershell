use axum::Router;
use clap::Parser;
use servershell::domain::marsweather::router::get_router;
use servershell::model::EndpointClient;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// Application arguments
struct Args {
    /// URL to Nasa Mars weather endpoint
    #[clap(env)]
    marsweather_endpoint: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    tracing_subscriber::fmt::init();
    let ec = EndpointClient::new(args.marsweather_endpoint.clone());
    let router_mars = get_router(ec);

    let app = Router::new().nest("/marsweather", router_mars).layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(CompressionLayer::new()),
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("able to setup TcpListener");
    axum::serve(listener, app)
        .await
        .expect("able to setup web server")
}
