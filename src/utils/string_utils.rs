pub fn to_snake_case(s: &str) -> String {
    let mut snake_case = String::new();
    let mut prev_char = '_';

    for c in s.chars() {
        if c.is_uppercase() && prev_char != '_' {
            snake_case.push('_');
        }
        snake_case.push(c.to_lowercase().next().unwrap());
        prev_char = c;
    }

    snake_case
}

pub fn to_camel_case(s: &str) -> String {
    let mut camel_case = String::new();
    let mut capitalize_next = false;

    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            camel_case.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            camel_case.push(c);
        }
    }

    camel_case
}
