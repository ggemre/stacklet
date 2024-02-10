#[cfg(test)]
mod filter_test {
    use stacklet::utils::filter::{exact_match, fuzzy_match};

    #[test]
    fn exact_match_positive() {
        assert!(exact_match("hello, world!", "o"));
        assert!(exact_match("foo", "foo"));
        assert!(exact_match("<>!;;", "!;;"));
        assert!(exact_match("heart", "ear"));
    }

    #[test]
    fn exact_match_negative() {
        assert!(!exact_match("hello, world!", "z"));
        assert!(!exact_match("foo", "bar"));
        assert!(!exact_match("<>!;;", "^"));
        assert!(!exact_match("rust", "rst"));
    }

    #[test]
    fn fuzzy_match_positive() {
        assert!(fuzzy_match("dogs and cats", "dgs n cts"));
        assert!(fuzzy_match("oython", "python"));
        assert!(fuzzy_match(
            "long sentence to search through",
            "swntemce to through"
        ));
        assert!(fuzzy_match("foobar", "foo..bAr.."));
    }

    #[test]
    fn fuzzy_match_negative() {
        assert!(!fuzzy_match("dogs and cats", "fphd smf vsyd"));
        assert!(!fuzzy_match("python", "nohtyp"));
        assert!(!fuzzy_match("big dogs dig deep holes", "abcdefghijklmnop"));
    }
}

#[cfg(test)]
mod helpers_test {
    use stacklet::utils::helpers::{find_widget_by_y, find_widget_by_y_mut};

    #[test]
    fn find_widget_by_y_negative() {
        assert!(find_widget_by_y(&Vec::new(), 0).is_none());
    }

    #[test]
    fn find_widget_by_y_mut_negative() {
        assert!(find_widget_by_y_mut(&mut Vec::new(), 0).is_none());
    }
}
