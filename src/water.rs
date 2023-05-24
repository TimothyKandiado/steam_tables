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
        pressure: f32,
        temperature: f32,
    ) -> Result<Vec<(String, String)>, Error> {
        let pressure_bounds = self.find_pressure_bounds(pressure);
        let temperature_bounds = self.find_temperature_bounds(temperature);

        let water_point_0_0 = self.get_water_point(pressure_bounds.0, temperature_bounds.0);
        let water_point_0_1 = self.get_water_point(pressure_bounds.0, temperature_bounds.1);
        let water_point_1_0 = self.get_water_point(pressure_bounds.1, temperature_bounds.0);
        let water_point_1_1 = self.get_water_point(pressure_bounds.1, temperature_bounds.1);

        let interpolated_water_point = interpolate_water_points(
            pressure,
            temperature,
            water_point_0_0,
            water_point_0_1,
            water_point_1_0,
            water_point_1_1,
        );

        Ok(self.convert_water_point_to_labelled_data(interpolated_water_point))
    }

    fn find_pressure_bounds(&self, target: f32) -> (f32, f32) {
        let mut lower_bound = self.get_minimum_allowable_pressure();
        let mut upper_bound = self.get_maximum_allowable_pressure();

        for value_point in &self.value_points {
            if value_point.point.0 <= target {
                lower_bound = value_point.point.0;
            } else if value_point.point.0 > target {
                upper_bound = value_point.point.0;
                break;
            }
        }

        (upper_bound, lower_bound)
    }

    fn get_water_point(&self, pressure: f32, temperature: f32) -> WaterPoint {
        let water_point = self
            .value_points
            .iter()
            .filter(|value_point| {
                value_point.point.0 == pressure && value_point.point.1 == temperature
            })
            .next();

        water_point.unwrap().clone()
    }

    fn find_temperature_bounds(&self, target: f32) -> (f32, f32) {
        let mut lower_bound = self.get_minimum_allowable_temperature();
        let mut upper_bound = self.get_maximum_allowable_temperature();

        for value_point in &self.value_points {
            if value_point.point.1 <= target {
                lower_bound = value_point.point.1;
            } else if value_point.point.1 > target {
                upper_bound = value_point.point.1;
                break;
            }
        }

        (upper_bound, lower_bound)
    }

    pub fn get_minimum_allowable_pressure(&self) -> f32 {
        self.value_points[0].point.0
    }

    pub fn get_maximum_allowable_pressure(&self) -> f32 {
        let value_length = self.value_points.len();
        self.value_points[value_length - 1].point.0
    }

    pub fn get_minimum_allowable_temperature(&self) -> f32 {
        self.value_points[0].point.1
    }

    pub fn get_maximum_allowable_temperature(&self) -> f32 {
        let value_length = self.value_points.len();
        self.value_points[value_length - 1].point.1
    }

    fn convert_water_point_to_labelled_data(
        &self,
        waterpoint: WaterPoint,
    ) -> Vec<(String, String)> {
        let mut labelled_data = Vec::new();
        let mut headers = self.headers.iter();

        labelled_data.push((
            headers.next().unwrap().to_owned(),
            waterpoint.point.0.to_string(),
        ));
        labelled_data.push((
            headers.next().unwrap().to_owned(),
            waterpoint.point.1.to_string(),
        ));

        waterpoint.values.iter().for_each(|value| {
            labelled_data.push((headers.next().unwrap().to_owned(), value.to_string()));
        });

        labelled_data.push((
            headers.next().unwrap().to_owned(),
            waterpoint.phase,
        ));

        labelled_data
    }
}
