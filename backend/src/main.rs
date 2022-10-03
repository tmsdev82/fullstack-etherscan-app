use warp::Filter;

mod client;
mod etherscan;

#[tokio::main]
async fn main() {
    log4rs::init_file("logging_config.yml", Default::default()).unwrap();

    let root = warp::path::end().map(|| "Ethereum transactions viewer app");
    let routes = root.with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 5000)).await;
}
