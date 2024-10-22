//! Definition for the different shapes that can be found in the IDF file.
//! https://docs.mantidproject.org/nightly/concepts/HowToDefineGeometricShape.html

use crate::Point;

/// Represents the different shapes that can be found in the IDF file.
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum Shapes {
    Cuboid(Cuboid),
    Hexahedron(Hexahedron),
}

/// Represents a hexahedron in the IDF file.
#[allow(missing_docs)]
#[derive(Debug, Default, Clone)]
pub struct Hexahedron {
    pub id: String,
    pub left_back_bottom_point: Point,
    pub left_front_bottom_point: Point,
    pub right_front_bottom_point: Point,
    pub right_back_bottom_point: Point,
    pub left_back_top_point: Point,
    pub left_front_top_point: Point,
    pub right_front_top_point: Point,
    pub right_back_top_point: Point,
}

impl Hexahedron {
    pub(crate) fn get_mut(&mut self, key: &[u8]) -> Option<&mut Point> {
        match key {
            b"left-back-bottom-point" => Some(&mut self.left_back_bottom_point),
            b"left-front-bottom-point" => Some(&mut self.left_front_bottom_point),
            b"right-front-bottom-point" => Some(&mut self.right_front_bottom_point),
            b"right-back-bottom-point" => Some(&mut self.right_back_bottom_point),
            b"left-back-top-point" => Some(&mut self.left_back_top_point),
            b"left-front-top-point" => Some(&mut self.left_front_top_point),
            b"right-front-top-point" => Some(&mut self.right_front_top_point),
            b"right-back-top-point" => Some(&mut self.right_back_top_point),
            _ => None,
        }
    }
}

/// Represents a cuboid in the IDF file.
#[allow(missing_docs)]
#[derive(Debug, Default, Clone)]
pub struct Cuboid {
    pub id: String,
    pub left_front_bottom_point: Point,
    pub left_front_top_point: Point,
    pub left_back_bottom_point: Point,
    pub right_front_bottom_point: Point,
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub centre: Point,
}

impl Cuboid {
    /// Create a new cuboid from the given points.
    pub fn from_points(
        id: String,
        left_front_bottom_point: Point,
        left_front_top_point: Point,
        left_back_bottom_point: Point,
        right_front_bottom_point: Point,
    ) -> Self {
        let width = nalgebra::distance(&left_front_bottom_point, &right_front_bottom_point);
        let height = nalgebra::distance(&left_front_bottom_point, &left_front_top_point);
        let depth = nalgebra::distance(&left_front_bottom_point, &left_back_bottom_point);
        let centre = Point::new(
            (left_front_bottom_point.x + right_front_bottom_point.x) / 2.0,
            (left_front_bottom_point.y + left_front_top_point.y) / 2.0,
            (left_front_bottom_point.z + left_back_bottom_point.z) / 2.0,
        );

        Self {
            id,
            left_front_bottom_point,
            left_front_top_point,
            left_back_bottom_point,
            right_front_bottom_point,
            width,
            height,
            depth,
            centre,
        }
    }

    /// Create a new cuboid from the given dimensions.
    pub fn from_dimensions(id: String, centre: Point, width: f32, height: f32, depth: f32) -> Self {
        let left_front_bottom_point = Point::new(
            centre.x - width / 2.0,
            centre.y - height / 2.0,
            centre.z - depth / 2.0,
        );
        let left_front_top_point = Point::new(
            centre.x - width / 2.0,
            centre.y + height / 2.0,
            centre.z - depth / 2.0,
        );
        let left_back_bottom_point = Point::new(
            centre.x - width / 2.0,
            centre.y - height / 2.0,
            centre.z + depth / 2.0,
        );
        let right_front_bottom_point = Point::new(
            centre.x + width / 2.0,
            centre.y - height / 2.0,
            centre.z - depth / 2.0,
        );

        Self {
            id,
            left_front_bottom_point,
            left_front_top_point,
            left_back_bottom_point,
            right_front_bottom_point,
            width,
            height,
            depth,
            centre,
        }
    }
}

#[cfg(test)]
mod test_cuboid {
    use super::*;

    #[test]
    fn test_cuboid_from_points() {
        let id = "cuboid".to_string();
        let left_front_bottom_point = Point::new(0.0, 0.0, 0.0);
        let left_front_top_point = Point::new(0.0, 1.0, 0.0);
        let left_back_bottom_point = Point::new(0.0, 0.0, 1.0);
        let right_front_bottom_point = Point::new(1.0, 0.0, 0.0);

        let cuboid = Cuboid::from_points(
            id.clone(),
            left_front_bottom_point,
            left_front_top_point,
            left_back_bottom_point,
            right_front_bottom_point,
        );

        assert_eq!(cuboid.id, id);
        assert_eq!(cuboid.width, 1.0);
        assert_eq!(cuboid.height, 1.0);
        assert_eq!(cuboid.depth, 1.0);
        assert_eq!(cuboid.centre, Point::new(0.5, 0.5, 0.5));
    }

    #[test]
    fn test_cuboid_from_dimensions() {
        let id = "cuboid".to_string();
        let centre = Point::new(0.5, 0.5, 0.5);
        let width = 1.0;
        let height = 1.0;
        let depth = 1.0;

        let cuboid = Cuboid::from_dimensions(id.clone(), centre, width, height, depth);

        assert_eq!(cuboid.id, id);
        assert_eq!(cuboid.width, 1.0);
        assert_eq!(cuboid.height, 1.0);
        assert_eq!(cuboid.depth, 1.0);
        assert_eq!(cuboid.centre, Point::new(0.5, 0.5, 0.5));
    }
}
