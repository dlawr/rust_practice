fn append_suffix(message: &mut String, suffix: &str) {
    if !message.ends_with(suffix) {
        message.push_str(suffix);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appends_suffix() {
        let mut s = String::from("report");
        append_suffix(&mut s, ".txt");
        assert_eq!(s, "report.txt");
    }

    #[test]
    fn does_not_duplicate_suffix() {
        let mut s = String::from("report.txt");
        append_suffix(&mut s, ".txt");
        assert_eq!(s, "report.txt");
    }

    #[test]
    fn handles_empty_string() {
        let mut s = String::new();
        append_suffix(&mut s, ".log");
        assert_eq!(s, ".log");
    }

    #[test]
    fn handles_empty_suffix() {
        let mut s = String::from("report");
        append_suffix(&mut s, "");
        assert_eq!(s, "report");
    }

    #[test]
    fn handles_unicode() {
        let mut s = String::from("café");
        append_suffix(&mut s, "☕");
        assert_eq!(s, "café☕");
    }
}
