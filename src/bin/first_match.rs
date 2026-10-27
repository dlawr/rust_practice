fn main() {
    first_match("rust is fun", &["java", "rust", "fun"]);
    // let s = "test string";
    // for (i, c) in s.char_indices() {
    //     let end_index = i + 2;
    //     let segment = s.get(i..end_index);
    //     println!("Byte {} is '{}'", i, c);
    //     println!("{:?}", segment);
    //     println!("{}", s.len());
    // }
}

fn first_match<'a>(text: &str, candidates: &[&'a str]) -> Option<&'a str> {
    for candidate in candidates {
        println!("{}", candidate);
        for (i, _c) in text.char_indices() {
            if let Some(window) = text.get(i..i + candidate.len()) {
                println!("{} {} {:?}", i, candidate, text.get(i..i + candidate.len()));
                if window.eq_ignore_ascii_case(candidate) {
                    return Some(candidate);
                }
            }
        }
        // if text.to_lowercase().contains(&candidate.to_lowercase()) {
        //     return Some(candidate);
        // }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_first_matching_candidate() {
        assert_eq!(
            first_match("rust is fun", &["java", "rust", "fun"]),
            Some("rust")
        );
    }

    #[test]
    fn matches_case_insensitive() {
        assert_eq!(first_match("Rust is fun", &["java", "rust"]), Some("rust"));
    }

    #[test]
    fn matches_case_insensitive_other_direction() {
        assert_eq!(first_match("rust is fun", &["java", "Rust"]), Some("Rust"));
    }

    #[test]
    fn respects_candidate_order() {
        assert_eq!(
            first_match("hello world", &["world", "hello"]),
            Some("world")
        );
    }

    #[test]
    fn returns_none_when_no_candidate_matches() {
        assert_eq!(first_match("hello", &["python", "java"]), None);
    }

    #[test]
    fn returns_none_for_empty_candidates() {
        assert_eq!(first_match("anything", &[]), None);
    }

    #[test]
    fn matches_empty_string() {
        assert_eq!(first_match("rust", &[""]), Some(""));
    }
}
