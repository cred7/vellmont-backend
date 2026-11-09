// pub fn current_timestamp() -> i64 {
//     chrono::Utc::now().timestamp()
// }

pub fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut previous_was_separator = true;

    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character.to_ascii_lowercase());
            previous_was_separator = false;
        } else if !previous_was_separator {
            slug.push('-');
            previous_was_separator = true;
        }
    }

    if slug.ends_with('-') {
        slug.pop();
    }

    slug
}

#[cfg(test)]
mod tests {
    use super::slugify;

    #[test]
    fn creates_lowercase_hyphenated_slugs() {
        assert_eq!(slugify("  Oak Crest / Apts "), "oak-crest-apts");
        assert_eq!(slugify("UNIT---A 101"), "unit-a-101");
        assert_eq!(slugify("!!!"), "");
    }
}
