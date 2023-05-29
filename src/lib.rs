pub mod data;
pub mod error;
pub mod saturated_steam;
pub mod water;

mod math;

fn get_headers_from_string(header_string: &str) -> Vec<String> {
    let headers: Vec<String> = header_string
        .split(',')
        .map(|header| header.trim().to_string())
        .collect();

    headers
}
