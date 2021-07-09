use rocket::response::content;
use rocket::State;
use rocket::{get, post};

use juniper::{EmptySubscription, RootNode};

use crate::graphql::schema::{Context, MutationRoot, QueryRoot};

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

#[get("/")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[get("/graphql?<request>")]
pub fn get_graphql_handler(
    context: Context,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
pub fn post_graphql_handler(
    context: Context,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}
