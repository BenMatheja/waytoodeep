use color_eyre::Report;
use tracing::info;
use tracing_subscriber::EnvFilter;
use reqwest::Client;
use std::future::Future;

pub const URL_1: &str = "https://fasterthanli.me/articles/whats-in-the-box";
pub const URL_2: &str = "https://fasterthanli.me/series/advent-of-code-2020/part-13";

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;
    let client = Client::new();
    let leaked_client = Box::leak(Box::new(client));
    let url: &str = "https://matheja.me";

    info!("Building the fetch futures...");
    let fut1 = fetch_thing(leaked_client, URL_1);
    let fut2 = fetch_thing(leaked_client, URL_2);
    let fut3 = fetch_thing(leaked_client, url);

    let handle1 = tokio::spawn(fut1);
    let handle2 = tokio::spawn(fut2);
    let handle3 = tokio::spawn(fut3);

    handle1.await.unwrap()?;
    handle2.await.unwrap()?;
    handle3.await.unwrap()?;

    Ok(())

}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    Ok(())
}

fn fetch_thing(
    client: &'static Client,
    url: &'static str,
) -> impl Future<Output = Result<(), Report>> + 'static {
    async move {
        let res = client.get(url).send().await?.error_for_status()?;
        info!(%url, content_type = ?res.headers().get("content-type"), "Got a response!");
        Ok(())
    }
}