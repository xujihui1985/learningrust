#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance_from_origin(self) -> f64 {
        self.x.hypot(self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_calculates_distance_from_origin() {
        let point = Point::new(3.0, 4.0);

        assert_eq!(point.distance_from_origin(), 5.0);
    }
}
