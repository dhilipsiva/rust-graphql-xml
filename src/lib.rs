use async_graphql::{InputObject, SimpleObject};
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(InputObject, YaSerialize, YaDeserialize, Debug)]
#[yaserde(rename = "MyData")]
pub struct MyInputData {
    pub id: i32,
    pub name: String,
}

#[derive(SimpleObject, YaSerialize, YaDeserialize, Debug)]
#[yaserde(rename = "MyData")]
pub struct MyOutputData {
    pub id: i32,
    pub name: String,
}
