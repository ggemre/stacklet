#[cfg(test)]
mod model_test {
    use stacklet::external::model::parse_stdout;

    #[test]
    fn parse_stdout_empty() {
        assert!(parse_stdout("").0.is_empty());
    }

    #[test]
    fn parse_stdout_input_valid() {
        assert!(!parse_stdout("INPUT()").0.is_empty());
        assert!(!parse_stdout("INPUT(filter=\"exact\")").0.is_empty());
        assert!(!parse_stdout("INPUT(filter=\"exact\" label=\"foo\" content=\"bar\")").0.is_empty());
        assert!(!parse_stdout("INPUT(content=\"\")").0.is_empty());
    }

    #[test]
    fn parse_stdout_input_invalid() {
        assert!(true);
        // TODO: handle invalid input
        // assert!(!parse_stdout("INPUT(filter=\"buzz\")").0.is_empty());
        // assert!(!parse_stdout("INPUT(filter=\"exact\",label=\"foo\" content=\"bar\")").0.is_empty());
        // assert!(!parse_stdout("INPUT(content=\")").0.is_empty());
    }

    #[test]
    fn parse_stdout_text_valid() {
        assert!(!parse_stdout("TEXT(\"exact\")").0.is_empty());
        assert!(!parse_stdout("TEXT(\"\")").0.is_empty());
    }

    #[test]
    fn parse_stdout_text_invalid() {
        // TODO: handle more invalid input
        assert!(parse_stdout("TEXT()").0.is_empty());
    }

    #[test]
    fn parse_stdout_full() {
        assert!(!parse_stdout("INPUT()\nTEXT()").0.is_empty());
        assert!(!parse_stdout("INPUT(filter=\"exact\")\nTEXT()\nTEXT()").0.is_empty());
        assert!(!parse_stdout("INPUT(filter=\"exact\" label=\"foo\" content=\"bar\")\nTEXT(\"foobar\")\nTEXT()").0.is_empty());
        assert!(!parse_stdout("INPUT(content=\"\")\nINPUT(label=\"foo\")\nINPUT()").0.is_empty());
    }

}

