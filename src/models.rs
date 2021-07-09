use crate::schema::Todo;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Insertable;
use juniper::{GraphQLEnum, GraphQLObject, GraphQLUnion};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaContent {
    pub class: String,
    pub url: String,
}
//#[derive(Debug, Clone, Serialize, Deserialize, GraphQLEnum)]
//pub enum MediaType {
//    Image,
//    Video,
//    Link,
//}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[table_name = "Todo"]
pub struct TodoEntry {
    pub id: i32,
    pub name: String,
    pub body: String,
    pub time_modified: NaiveDateTime,
    pub completed: bool,
    pub media: Vec<MediaContent>,
    pub parent_id: i32,
    pub parent_name: String,
}
