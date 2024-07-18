#[derive(Debug, Clone, PartialEq)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl Resolution {
    pub fn new(width: u32, height: u32) -> Resolution {
        Resolution { width, height }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_WIDTH: u32 = 1920;
    const TEST_HEIGHT: u32 = 1080;

    #[test]
    fn test_new() {
        let resolution = Resolution::new(TEST_WIDTH, TEST_HEIGHT);

        assert_eq!(resolution.width, TEST_WIDTH);
        assert_eq!(resolution.height, TEST_HEIGHT);
    }
}
