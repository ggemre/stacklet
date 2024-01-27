use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Filter {
    Off,
    Exact,
    Fuzzy,
}

impl FromStr for Filter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "off" => Ok(Filter::Off),
            "exact" => Ok(Filter::Exact),
            "fuzzy" => Ok(Filter::Fuzzy),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Widget {
    Input {
        y: i32,
        max_width: usize,
        filter: Filter,
        label: String,
        placeholder: String,
        content: String,
        id: usize,
    },
    Text {
        y: i32,
        content: String,
        show: bool,
        id: usize,
    },
}
