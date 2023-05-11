#[cfg(test)]
mod tests {

    use genpdf;
    use serde_json::{Value, json};

    #[test]
    fn test_api() {
        genpdf::genpdf("test.typ".into(), "test.pdf".into(),Some(".".into()), None);
    }

    #[test]
    fn test_api_with_json() {
        let my_value: Value = json!({
            "name": "John",
            "age": 30,
            "is_student": true,
            "hobbies": ["reading", "coding", "hiking"],
            "address": {
                "street": "123 Main St",
                "city": "Anytown",
                "state": "CA",
                "zip": "12345"
            }
        });

        genpdf::genpdf("test_with_json.typ".into(), "test_with_json.pdf".into(),Some(".".into()), Some(my_value));

    }

}
