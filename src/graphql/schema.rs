use crate::db::Connection;
use juniper::{
    graphql_interface, graphql_object, graphql_subscription, Context as JuniperContext, GraphQLEnum,
};
pub struct Context {
    connection: Connection,
}
impl JuniperContext for Context {}

pub struct QueryRoot;
#[graphql_object(context = Context)]
impl QueryRoot {}

pub struct MutationRoot;
#[graphql_object(context = Context)]
impl MutationRoot {}
