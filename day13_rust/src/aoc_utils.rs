pub fn parse_field(text: &str) -> i64 {
        match text.parse() {
            Ok(i) => i,
            Err(_) => {
                panic!("Unreadable number {}", text);
            }
        }
    }
