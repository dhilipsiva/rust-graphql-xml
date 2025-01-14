use async_graphql::{Context, Object};
use rust_graphql_xml::{MyInputData, MyOutputData};
use std::fs;
use std::io::Write;
use yaserde::de::from_str;
use yaserde::ser::to_string;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn read_data(&self, _ctx: &Context<'_>) -> async_graphql::Result<MyOutputData> {
        let xml_content = fs::read_to_string("data.xml")
            .map_err(|e| format!("Failed to read outgoing_data.xml: {}", e))?;
        let my_data: MyOutputData =
            from_str(&xml_content).map_err(|e| format!("XML deserialization error: {}", e))?;
        Ok(my_data)
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
        let xml_string = to_string(&data).map_err(|e| format!("XML serialization error: {}", e))?;
        let mut file =
            fs::File::create("data.xml").map_err(|e| format!("Cannot create data.xml: {}", e))?;
        file.write_all(xml_string.as_bytes())
            .map_err(|e| format!("Cannot write to file: {}", e))?;
        Ok(true)
    }
}
