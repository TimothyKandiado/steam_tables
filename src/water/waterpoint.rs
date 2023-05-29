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
            let point0 = values.next().unwrap().trim().parse::<f32>();
            let point1 = values.next().unwrap().trim().parse::<f32>();

            let mut point_values: Vec<f32> = Vec::new();
            let mut phase = String::new();

            for point_value in values {
                let float_value = point_value.trim().parse::<f32>();

                if let Ok(value) = float_value {
                    point_values.push(value);
                } else {
                    phase = point_value.to_owned().to_string();
                }
            }

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
    water_point_1_1: WaterPoint,
) -> WaterPoint {
    let number_of_values = water_point_0_0.values.len();

    let values: Vec<f32> = (0..number_of_values)
        .map(|index| {
            let point_0_0 = Point3(
                water_point_0_0.point.0,
                water_point_0_0.point.1,
                water_point_0_0.values[index],
            );

            let point_0_1 = Point3(
                water_point_0_1.point.0,
                water_point_0_1.point.1,
                water_point_0_1.values[index],
            );

            let point_1_0 = Point3(
                water_point_1_0.point.0,
                water_point_1_0.point.1,
                water_point_1_0.values[index],
            );

            let point_1_1 = Point3(
                water_point_1_1.point.0,
                water_point_1_1.point.1,
                water_point_1_1.values[index],
            );

            double_linear_interpolate(
                pressure,
                temperature,
                point_0_0,
                point_0_1,
                point_1_0,
                point_1_1,
            )
        })
        .collect();

    WaterPoint {
        point: (pressure, temperature),
        values,
        phase: water_point_0_0.phase,
    }
}

pub fn phase_change_occurs(water_points: (WaterPoint, WaterPoint, WaterPoint, WaterPoint)) -> bool {
    if water_points.0.phase != water_points.1.phase
        || water_points.0.phase != water_points.2.phase
        || water_points.0.phase != water_points.3.phase
    {
        return true;
    }

    false
}

pub fn get_nearest_water_point(
    pressure: f32,
    temperature: f32,
    water_points: (WaterPoint, WaterPoint, WaterPoint, WaterPoint),
) -> WaterPoint {
    let mut nearest = water_points.0.clone();
    let mut cur_score =
        (nearest.point.0 - pressure).powi(2) + (nearest.point.1 - temperature).powi(2);

    let new_score = (water_points.1.point.0 - pressure).powi(2)
        + (water_points.1.point.1 - temperature).powi(2);
    if new_score < cur_score {
        nearest = water_points.1.clone();
        cur_score = new_score;
    }

    let new_score = (water_points.2.point.0 - pressure).powi(2)
        + (water_points.2.point.1 - temperature).powi(2);
    if new_score < cur_score {
        nearest = water_points.2.clone();
        cur_score = new_score;
    }

    let new_score = (water_points.3.point.0 - pressure).powi(2)
        + (water_points.3.point.1 - temperature).powi(2);
    if new_score < cur_score {
        nearest = water_points.3;
    }

    nearest
}
