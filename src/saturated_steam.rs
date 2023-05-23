use crate::error::Error;
use datapoint::*;

mod datapoint;

pub struct SteamTable {
    headers: Vec<String>,
    datapoints: Vec<DataPoint>,
}

impl SteamTable {
    pub fn new(data_table: String) -> Result<SteamTable, Error> {
        let data_lines: Vec<&str> = data_table.lines().collect();
        println!("loaded {}", data_lines[0]);

        let headers: Vec<String> = super::get_headers_from_string(&data_lines[6]);
        let str_data: Vec<String> = data_lines[7..].iter().map(|str| str.to_string()).collect();

        let mut datapoints = parse_to_datapoint_struct(str_data)?;
        datapoints.sort_by(|a, b| a.point.partial_cmp(&b.point).unwrap());

        let steam_table = SteamTable {
            headers,
            datapoints,
        };

        Ok(steam_table)
    }

    pub fn get_values_at_point(&self, point: f32) -> Result<Vec<(String, f32)>, Error> {
        self.is_point_valid(point)?;

        let (min_data_point, max_data_point) = self.get_bounding_points(point);

        let data_point = interpolate_data_points(point, min_data_point, max_data_point);

        Ok(self.merge_header_with_data_point(data_point))
    }

    fn get_bounding_points(&self, point: f32) -> (DataPoint, DataPoint) {
        let mut lower_bound = self.smallest_valid_point();
        let mut upper_bound = self.largest_valid_point();

        let mut datapoint_iterator = self.datapoints.iter();
        while let Some(datapoint) = datapoint_iterator.next() {
            if datapoint.point <= point {
                lower_bound = datapoint.point
            } else if datapoint.point > point {
                upper_bound = datapoint.point;
                break;
            }
        }

        let upper_point = self.get_data_point(upper_bound);
        let lower_point = self.get_data_point(lower_bound);

        (lower_point.unwrap(), upper_point.unwrap())
    }

    fn get_data_point(&self, point: f32) -> Option<DataPoint> {
        let mut data_point = self
            .datapoints
            .iter()
            .filter(|data_point| data_point.point == point);

        if let Some(point) = data_point.next() {
            Some(point.clone())
        } else {
            None
        }
    }

    fn merge_header_with_data_point(&self, data_point: DataPoint) -> Vec<(String, f32)> {
        let mut labelled_data: Vec<(String, f32)> = Vec::new();

        labelled_data.push((self.headers[0].clone(), data_point.point));

        let mut tail_data: Vec<(String, f32)> = self.headers[1..]
            .iter()
            .zip(data_point.values)
            .map(|pair| (pair.0.clone(), pair.1))
            .collect();

        labelled_data.append(&mut tail_data);

        labelled_data
    }

    pub fn is_point_valid(&self, point: f32) -> Result<(), Error> {
        let smallest_point = self.smallest_valid_point();
        let largest_point = self.largest_valid_point();

        if point < smallest_point {
            return Err(Error::ValueOutOfRange(smallest_point, largest_point));
        } else if point > largest_point {
            return Err(Error::ValueOutOfRange(smallest_point, largest_point));
        }

        Ok(())
    }

    pub fn smallest_valid_point(&self) -> f32 {
        let smallest_point = self.datapoints[0].point;
        smallest_point
    }

    pub fn largest_valid_point(&self) -> f32 {
        let datapoints_length = self.datapoints.len();
        let largest_point = self.datapoints[datapoints_length - 1].point;

        largest_point
    }
}
