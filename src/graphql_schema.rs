use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Context, EmptySubscription, Object, Schema,
};
use async_graphql_warp::{graphql, GraphQLResponse};
use rust_graphql_xml::{read_data_from_file, write_data_to_file, MyInputData, MyOutputData};
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn read_data(&self, _ctx: &Context<'_>) -> async_graphql::Result<MyOutputData> {
        read_data_from_file().map_err(|e| e.into())
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn write_data(
        &self,
        _ctx: &Context<'_>,
        data: MyInputData,
    ) -> async_graphql::Result<bool> {
        write_data_to_file(&data).map_err(async_graphql::Error::new)?;

        Ok(true)
    }
}

pub fn create_graphql_schema() -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish()
}

pub fn get_graphql_routes(
    schema: Schema<QueryRoot, MutationRoot, EmptySubscription>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let graphql_filter = graphql(schema).and_then(
        |(schema, request): (Schema<QueryRoot, MutationRoot, _>, async_graphql::Request)| async move {
            let resp = schema.execute(request).await;
            Ok::<_, Infallible>(GraphQLResponse::from(resp))
        },
    );
    let playground_route = warp::path("playground").and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
    });
    let graphql_route = warp::path("graphql").and(graphql_filter);
    playground_route.or(graphql_route)
}
