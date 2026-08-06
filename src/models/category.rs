#[derive(Debug, Clone)]
pub struct Category {
    pub id: i64,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn category_fields() {
        let c = Category {
            id: 42,
            name: "Work".to_string(),
        };
        assert_eq!(c.id, 42);
        assert_eq!(c.name, "Work");
    }
}
