pub fn linear_interpolate(x: f32, point0: Point2, point1: Point2) -> f32 {
    

    point0.1 + (x - point0.0) * (point1.1 - point0.1) / (point1.0 - point0.0)
}

/// Used when a value depends upon two values x, and y
/// Point3 struct contains known value at x and y in the format
/// x, y, u
/// where u is the value of function
/// x and y are the independent variables
/// coordinates
/// point_0_1 ---------- ---------- point_1_1
///     |               |               |
/// point_0_y ------ point_x_y ---- point_1_y
///     |               |               |
/// point_0_0 ---------- ---------- point_1_0
pub fn double_linear_interpolate(
    x: f32,
    y: f32,
    point_0_0: Point3,
    point_0_1: Point3,
    point_1_0: Point3,
    point_1_1: Point3,
) -> f32 {
    // intepolate between point_0_0 and point_0_1 to find point_0_y
    // x remains constant thus only y is intepolated

    let point_0_y_value = linear_interpolate(
        y,
        Point2(point_0_0.1, point_0_0.2),
        Point2(point_0_1.1, point_0_1.2),
    );

    let point_1_y_value = linear_interpolate(
        y,
        Point2(point_1_0.1, point_1_0.2),
        Point2(point_1_1.1, point_1_1.2),
    );

    // interpolate between point_0_y and point_1_y to find value of point_x_y
    // y remains constant so we'll interpolate along x only

    linear_interpolate(
        x,
        Point2(point_0_0.0, point_0_y_value),
        Point2(point_1_0.0, point_1_y_value),
    )
}

#[derive(Debug, Clone, Copy)]
pub struct Point2(pub f32, pub f32);
#[derive(Debug, Clone, Copy)]
pub struct Point3(pub f32, pub f32, pub f32);

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;
    #[test]
    fn test_linear_interpolate() {
        let point0 = Point2(0.0, 0.0);
        let point1 = Point2(1.0, 1.0);

        let x = 0.5;
        let y = linear_interpolate(x, point0, point1);

        let x1 = 1.0;
        let y1 = linear_interpolate(x1, point0, point1);

        assert_eq!(y, 0.5);
        assert_eq!(y1, 1.0);
        assert_eq!(linear_interpolate(0.0, point0, point1), 0.0);
    }

    #[test]
    fn test_double_linear_interpolate() {
        let point_0_0 = Point3(5.0, 100.0, 2000.0);
        let point_0_1 = Point3(5.0, 200.0, 2020.0);
        let point_1_0 = Point3(15.0, 100.0, 1900.0);
        let point_1_1 = Point3(15.0, 200.0, 1920.0);

        let (x, y) = (10.0, 150.0); // interpolation points
        let u = 1960.0; // final answer

        assert_eq!(
            double_linear_interpolate(x, y, point_0_0, point_0_1, point_1_0, point_1_1),
            u
        );
    }
}
