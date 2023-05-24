use crate::error::Error;
use crate::math::*;

#[derive(Debug, Clone)]
pub struct WaterPoint {
    /// point (Pressure, Temperature)
    pub point: (f32, f32),
    pub values: Vec<f32>,
    pub phase: String,
}

pub fn parse_to_water_point_struct(lines: Vec<String>) -> Result<Vec<WaterPoint>, Error> {
    let water_points: Vec<WaterPoint> = lines
        .iter()
        .map(|line| {
            let mut values = line.split(',');
            let point0 = values.next().unwrap().parse::<f32>();
            let point1 = values.next().unwrap().parse::<f32>();

            let point_values: Vec<f32> = values
                .clone()
                .filter_map(|value| value.parse::<f32>().ok())
                .collect();

            let phase = values.last().unwrap().to_string();

            WaterPoint {
                point: (point0.unwrap(), point1.unwrap()),
                values: point_values,
                phase,
            }
        })
        .collect();

    Ok(water_points)
}

pub fn interpolate_water_points(
    pressure: f32, 
    temperature: f32,
    water_point_0_0: WaterPoint,
    water_point_0_1: WaterPoint,
    water_point_1_0: WaterPoint,
    water_point_1_1: WaterPoint
    ) -> WaterPoint {

        let number_of_values = water_point_0_0.values.len();

        let values: Vec<f32> = (0..number_of_values).into_iter().map(
            |index| {
                let point_0_0 = Point3(
                    water_point_0_0.point.0, 
                    water_point_0_0.point.1, 
                    water_point_0_0.values[index]);

                let point_0_1 = Point3(
                    water_point_0_1.point.0, 
                    water_point_0_1.point.1, 
                    water_point_0_1.values[index]);

                let point_1_0 = Point3(
                    water_point_1_0.point.0, 
                    water_point_1_0.point.1, 
                    water_point_1_0.values[index]);

                let point_1_1 = Point3(
                    water_point_1_1.point.0, 
                    water_point_1_1.point.1, 
                    water_point_1_1.values[index]);

            double_linear_interpolate(pressure, temperature, point_0_0, point_0_1, point_1_0, point_1_1)
            }
        ).collect();

        WaterPoint { point: (pressure, temperature),
             values, 
             phase: water_point_0_0.phase
             }
}