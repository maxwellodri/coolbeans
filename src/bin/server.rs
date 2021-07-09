#![feature(decl_macro)]
use coolbeans::db::{self, PgPool};
use coolbeans::routes;
use dotenv::dotenv;
//use juniper::{EmptyMutation, EmptySubscription, RootNode};
#[macro_use]
extern crate rocket;
use rocket::{response::content, routes, State};

//type Schema = RootNode<'static, Query, EmptyMutation<Database>, EmptySubscription<Database>>;
//
#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

//#[rocket::get("/graphql?<request>")]
//fn get_graphql_handler(
//    context: db::Connection,
//    request: juniper_rocket::GraphQLRequest,
//    schema: State<Schema>,
//) -> juniper_rocket::GraphQLResponse {
//    request.execute_sync(&schema, &context)
//}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    //context: State<db::Connection>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
    api_key: ApiKey,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&schema, &context)
}
#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/", routes![graphiql])
}
