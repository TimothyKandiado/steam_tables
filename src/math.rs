pub fn linear_interpolate(x: f32, point0: Point, point1: Point) -> f32{
    let y = point0.1 + (x - point0.0) * (point1.1 - point0.1) / (point1.0 - point0.0);

    y
}

#[derive(Debug, Clone, Copy)]
pub struct Point (pub f32, pub f32);

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;
    #[test]
    fn test_linear_interpolate() {
        let point0 = Point(0.0, 0.0);
        let point1 = Point(1.0, 1.0);

        let x = 0.5;
        let y = linear_interpolate(x, point0, point1);

        let x1 = 1.0;
        let y1 = linear_interpolate(x1, point0, point1);

        assert_eq!(y, 0.5);
        assert_eq!(y1, 1.0);
        assert_eq!(linear_interpolate(0.0, point0, point1), 0.0);
    }
}