use bytes::Bytes;
use rust_graphql_xml::{read_data_from_file, write_data_to_file, MyInputData, MyOutputData};
use std::str;
use warp::{http::Response as HttpResponse, Filter};
use yaserde::{de::from_str, ser::to_string};

pub fn get_xml_routes(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let xml_post = warp::path("xml")
        .and(warp::post())
        .and(warp::header::exact("content-type", "application/xml"))
        .and(warp::body::bytes())
        .map(|body: Bytes| {
            let input_str = match str::from_utf8(&body) {
                Ok(s) => s,
                Err(e) => {
                    return HttpResponse::builder()
                        .status(400)
                        .body(format!("Invalid UTF-8 in request body: {e}"));
                }
            };
            let parsed: MyInputData = match from_str(input_str) {
                Ok(d) => d,
                Err(e) => {
                    return HttpResponse::builder()
                        .status(400)
                        .body(format!("XML deserialization error: {e}"));
                }
            };
            if let Err(err) = write_data_to_file(&parsed) {
                return HttpResponse::builder()
                    .status(500)
                    .body(format!("Failed to write data.xml: {err}"));
            }
            let output = MyOutputData {
                id: parsed.id,
                name: parsed.name,
            };
            let out_xml = match to_string(&output) {
                Ok(s) => s,
                Err(e) => {
                    return HttpResponse::builder()
                        .status(500)
                        .body(format!("XML serialization error: {e}"));
                }
            };

            HttpResponse::builder()
                .header("content-type", "application/xml")
                .body(out_xml)
        });

    let xml_get = warp::path("xml").and(warp::get()).map(|| {
        let output = match read_data_from_file() {
            Ok(data) => data,
            Err(e) => {
                return HttpResponse::builder()
                    .status(500)
                    .body(format!("Failed to read data.xml: {e}"));
            }
        };

        let out_xml = match to_string(&output) {
            Ok(xml) => xml,
            Err(e) => {
                return HttpResponse::builder()
                    .status(500)
                    .body(format!("XML serialization error: {e}"));
            }
        };

        HttpResponse::builder()
            .header("content-type", "application/xml")
            .body(out_xml)
    });

    xml_post.or(xml_get)
}
