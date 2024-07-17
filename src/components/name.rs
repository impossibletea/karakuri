#[derive(Debug, Clone, PartialEq)]
pub struct Name {
    value: String,
}

impl Name {
    pub fn new(value: String) -> Name {
        Name { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::Name;

    #[test]
    fn test_new() {
        let snake = String::from("Solid Snake");

        let name = Name::new(snake.clone());

        assert_eq!(name.value, snake);
    }
}
