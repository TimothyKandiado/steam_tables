use crate::error::Error;

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
