#![feature(decl_macro)]
use crate::routes::{get_graphql_handler, graphiql, post_graphql_handler};
use coolbeans::db::{self, PgPool};
use coolbeans::routes;
use dotenv::dotenv;
use rocket::routes;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount(
        "/",
        routes![graphiql, get_graphql_handler, post_graphql_handler],
    )
}
