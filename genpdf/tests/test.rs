#[cfg(test)]
mod tests {

    use genpdf;
    use serde_json::{Value, json};
    use std::fs;

    #[test]
    fn test_api() {
        if let Ok(data) = genpdf::genpdf("test.typ".into(), "test.pdf".into(),".".into(), None) {
            fs::write("test.pdf", data);
        } else {
            panic!("Test failed");

        }
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

        
        if let Ok(data) = genpdf::genpdf("test_with_json.typ".into(), "test_with_json.pdf".into(),".".into(), Some(my_value)) {
            fs::write("test_with_json.pdf", data);
        } else {
            panic!("Test failed");

        }
    }

}
