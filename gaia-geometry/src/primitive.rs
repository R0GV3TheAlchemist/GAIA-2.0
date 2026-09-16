//! Formal geometry only. Circles do not command nature.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance(self, other: Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Result<Self, String> {
        if radius <= 0.0 {
            return Err("radius must be positive".into());
        }
        Ok(Self {
            center: Point::new(x, y),
            radius,
        })
    }

    pub fn contains(self, p: Point) -> bool {
        self.center.distance(p) <= self.radius
    }

    /// Context boundary. Not a magic circle.
    pub fn is_command(&self) -> bool {
        false
    }
}

/// Intersection of two equal-radius circles: shared domain, not a portal.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vesica {
    pub left: Circle,
    pub right: Circle,
}

impl Vesica {
    pub fn try_new(left: Circle, right: Circle) -> Result<Self, String> {
        if (left.radius - right.radius).abs() > 1e-9 {
            return Err("vesica fixture requires equal radii".into());
        }
        let d = left.center.distance(right.center);
        if d <= 0.0 || d >= left.radius + right.radius {
            return Err("circles do not form a vesica piscis".into());
        }
        Ok(Self { left, right })
    }

    pub fn overlap_width(&self) -> f64 {
        let d = self.left.center.distance(self.right.center);
        (self.left.radius + self.right.radius - d).max(0.0)
    }
}
