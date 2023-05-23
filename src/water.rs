use crate::error::Error;

mod waterpoint;
use waterpoint::*;

/// Table to hold properties of water at different temperatures and pressure
pub struct WaterTable {
    headers: Vec<String>,
    value_points: Vec<WaterPoint>,
}

impl WaterTable {
    pub fn new(data_table: String) -> Result<WaterTable, Error> {
        let lines: Vec<String> = data_table.lines().map(|line| line.to_string()).collect();

        let headers = super::get_headers_from_string(&lines[6]);
        let value_lines = lines[7..].to_vec();

        let value_points = parse_to_water_point_struct(value_lines)?;

        Ok(WaterTable {
            headers,
            value_points,
        })
    }

    pub fn get_values_at_point(
        &self,
        _pressure: f32,
        _temperature: f32,
    ) -> Result<WaterPoint, Error> {
        Err(Error::ValueOutOfRange(0.0, 0.0))
    }
}
