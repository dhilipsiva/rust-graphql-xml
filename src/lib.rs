use async_graphql::{InputObject, SimpleObject};
use std::{fs, io::Write};
use yaserde::de::from_str;
use yaserde::ser::to_string;
use yaserde_derive::{YaDeserialize, YaSerialize};

pub const DATA_FILE: &str = "data.xml";

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

pub fn write_data_to_file(input: &MyInputData) -> Result<(), String> {
    let xml_string = to_string(input).map_err(|e| e.to_string())?;
    let mut file = fs::File::create(DATA_FILE).map_err(|e| e.to_string())?;
    file.write_all(xml_string.as_bytes())
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn read_data_from_file() -> Result<MyOutputData, String> {
    let xml_str = fs::read_to_string(DATA_FILE).map_err(|e| e.to_string())?;
    from_str::<MyOutputData>(&xml_str).map_err(|e| e.to_string())
}
