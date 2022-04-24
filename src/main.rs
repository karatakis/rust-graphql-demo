use rust_graphql_demo::database_schema::get_database_schema;
use rust_graphql_demo::graphql_schema::query::get_graphql_schema;
use sqlx::SqlitePool;
use warp::{http::Response, Filter};

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "warp_server");
    env_logger::init();
    let log = warp::log("warp_server");

    let connection = SqlitePool::connect("sqlite://chinook.db").await.unwrap();

    let database_schema = get_database_schema(connection).await.unwrap();

    let graphql_schema = get_graphql_schema(&database_schema).await;

    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(format!(
                "<html><h1>juniper_warp</h1><div>visit <a href=\"/playground\">/playground</a></html>"
            ))
    });

    let state = warp::any().map(move || { () });
    let graphql_filter = juniper_warp::make_graphql_filter(graphql_schema, state.boxed());

    warp::serve(
        warp::get()
            .and(warp::path("playground"))
            .and(juniper_warp::playground_filter("/graphql", None))
            .or(homepage)
            .or(warp::path("graphql").and(graphql_filter))
            .with(log),
    )
    .run(([127, 0, 0, 1], 8090))
    .await
}
