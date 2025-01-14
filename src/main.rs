mod graphql_schema;
mod xml_schema;

use graphql_schema::{create_graphql_schema, get_graphql_routes};
use warp::Filter;
use xml_schema::get_xml_routes;

#[tokio::main]
async fn main() {
    let schema = create_graphql_schema();
    let graphql_routes = get_graphql_routes(schema);
    let xml_routes = get_xml_routes();
    let routes = graphql_routes.or(xml_routes);
    println!("GraphQL Playground: http://localhost:8000/playground");
    println!("GraphQL endpoint:   http://localhost:8000/graphql");
    println!("XML endpoint:       POST or GET http://localhost:8000/xml");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
