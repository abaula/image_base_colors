use axum::extract::Multipart;
use std::collections::HashMap;

pub struct Request {
    pub number_of_clusters: u32,
    pub max_try_count: u32,
    pub file_name: String,
    pub file_buffer: Vec<u8>,
}

impl Request {
    pub async fn parse(
        params: &HashMap<String, String>,
        multipart: &mut Multipart,
    ) -> Result<Request, String> {
        let number_of_clusters =
            get_number_of_clusters(&params).unwrap_or(get_number_of_clusters_default());

        let max_try_count = get_max_try_count(&params).unwrap_or(get_max_try_count_default());

        let (name, buffer) = match get_image_buffer(multipart).await {
            Some(value) => value,
            None => return Err(String::from("Can't read image from request.")),
        };

        Ok(Request {
            number_of_clusters,
            max_try_count,
            file_name: name,
            file_buffer: buffer,
        })
    }
}

fn get_number_of_clusters_default() -> u32 {
    4
}

fn get_max_try_count_default() -> u32 {
    30
}

fn get_number_of_clusters(params: &HashMap<String, String>) -> Option<u32> {
    const FIELD_NAME: &str = "number_of_clusters";

    get_filed_value_u32(params, FIELD_NAME)
}

fn get_max_try_count(params: &HashMap<String, String>) -> Option<u32> {
    const FIELD_NAME: &str = "max_try_count";

    get_filed_value_u32(params, FIELD_NAME)
}

async fn get_image_buffer(multipart: &mut Multipart) -> Option<(String, Vec<u8>)> {
    let field_opt = match multipart.next_field().await {
        Ok(value) => value,
        Err(err) => {
            println!("Error: {err}");
            return None;
        }
    };

    let field = field_opt?;

    let name = match field.name() {
        Some(value) => String::from(value),
        None => String::new(),
    };

    let buffer = match field.bytes().await {
        Ok(value) => value.to_vec(),
        Err(err) => {
            println!("Error: {err}");
            return None;
        }
    };

    Some((name, buffer))
}

fn get_filed_value_u32(params: &HashMap<String, String>, field_name: &str) -> Option<u32> {
    let field_value_str = match params.get(field_name) {
        Some(value) => value,
        None => {
            println!("Param '{field_name}' not found.");
            return None;
        }
    };

    match field_value_str.parse::<u32>() {
        Ok(number) => Some(number),
        Err(err) => {
            println!("Parse error: {err}");
            None
        }
    }
}
