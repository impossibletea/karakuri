#[derive(Debug, Clone, PartialEq)]
pub struct Resolution {
    pub width: u64,
    pub height: u64,
}

impl Resolution {
    pub fn new(width: u64, height: u64) -> Resolution {
        Resolution { width, height }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_WIDTH: u64 = 1920;
    const TEST_HEIGHT: u64 = 1080;

    #[test]
    fn test_new() {
        let resolution = Resolution::new(TEST_WIDTH, TEST_HEIGHT);

        assert_eq!(resolution.width, TEST_WIDTH);
        assert_eq!(resolution.height, TEST_HEIGHT);
    }
}
