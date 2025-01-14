mod schema;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::Schema;
use async_graphql_warp::{graphql, GraphQLResponse};
use schema::{MutationRoot, QueryRoot};
use warp::{http::Response as HttpResponse, Filter};

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, MutationRoot, async_graphql::EmptySubscription).finish();
    let graphql_filter = graphql(schema).and_then(
        |(schema, request): (Schema<QueryRoot, MutationRoot, _>, async_graphql::Request)| async move {
            let resp = schema.execute(request).await;
            Ok::<_, std::convert::Infallible>(GraphQLResponse::from(resp))
        },
    );
    let playground_route = warp::path("playground").and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
    });
    let routes = playground_route.or(warp::path("graphql").and(graphql_filter));
    println!("GraphQL Playground: http://localhost:8000/playground");
    println!("GraphQL endpoint:   http://localhost:8000/graphql");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
