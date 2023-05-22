use crate::error::Error;
use super::math::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DataPoint {
    pub point: f32,
    pub values: Vec<f32>,
}

pub fn parse_to_datapoint_struct(lines: Vec<String>) -> Result<Vec<DataPoint>, Error> {
    let datapoints : Vec<DataPoint>;

    datapoints = lines.iter().map(|line| {
        let mut data_points = line.split(",").map(|value| {
            value.parse::<f32>().expect("could not parse float")
        });

        let point = data_points.next().expect("no value taken");
        let values: Vec<f32> = data_points.map(|value| value).collect();

        DataPoint {
            point,
            values
        }
    }).collect();

    Ok(datapoints)
}

pub fn interpolate_data_points(point: f32, min_data_point: DataPoint, max_data_point: DataPoint) -> DataPoint {
    if min_data_point == max_data_point {
        return min_data_point;
    }
    
    let lower_bound = min_data_point.point;
    let upper_bound = max_data_point.point;


    let lower_bound_values = min_data_point.values;
    let upper_bound_values = max_data_point.values;

    let values = lower_bound_values.iter()
        .zip(upper_bound_values)
        .map(|pair| {
            let point0 = Point (lower_bound, *pair.0);

            let point1 = Point(upper_bound, pair.1);

            let value = linear_interpolate(point, point0, point1);

            value
        }).collect();


    DataPoint { point, values }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_interpolate_data_points() {
        let data_point_0 = DataPoint {
            point: 0.0,
            values: vec![0.0,0.0,0.0,0.0,0.0],
        };

        let data_point_1 = DataPoint {
            point: 1.0,
            values: vec![1.0,1.0,1.0,1.0,1.0],
        };

        let data_point_mid = DataPoint {
            point: 0.5,
            values: vec![0.5,0.5,0.5,0.5,0.5],
        };

        let lerp1 = interpolate_data_points(0.5, data_point_0.clone(), data_point_1.clone());
        assert_eq!(lerp1, data_point_mid);

        let lerp2 = interpolate_data_points(0.0, data_point_0.clone(), data_point_0.clone());
        assert_eq!(lerp2, data_point_0);
    }
}