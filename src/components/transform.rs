use crate::math::Vector2;

#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    pub position: Vector2,
    pub scale: Vector2,
    pub rotation: f64,
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            position: Vector2::ZERO,
            scale: Vector2::new(1., 1.),
            rotation: 0.,
        }
    }
}

impl Transform {
    pub fn new(position: Vector2, scale: Vector2, rotation: f64) -> Transform {
        Transform {
            position,
            scale,
            rotation,
        }
    }

    pub fn from_position(position: Vector2) -> Transform {
        Transform {
            position,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_POSITION: Vector2 = Vector2 { x: 23., y: 88. };
    const TEST_SCALE: Vector2 = Vector2 { x: 1., y: 2. };
    const TEST_ROTATION: f64 = 1.5078;

    #[test]
    fn test_new() {
        let transform = Transform::new(TEST_POSITION, TEST_SCALE, TEST_ROTATION);

        assert_eq!(transform.position, TEST_POSITION);
        assert_eq!(transform.scale, TEST_SCALE);
        assert_eq!(transform.rotation, TEST_ROTATION);
    }

    #[test]
    fn test_from_position() {
        let transform = Transform::from_position(TEST_POSITION);
        let default_transform = Transform::default();

        assert_eq!(transform.position, TEST_POSITION);
        assert_eq!(transform.scale, default_transform.scale);
        assert_eq!(transform.rotation, default_transform.rotation);
    }

    #[test]
    fn test_default() {
        let transform = Transform::default();

        assert_eq!(transform.position, Vector2::new(0., 0.));
        assert_eq!(transform.scale, Vector2::new(1., 1.));
        assert_eq!(transform.rotation, 0.);
    }
}
