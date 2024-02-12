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
            | "off" => Ok(Filter::Off),
            | "exact" => Ok(Filter::Exact),
            | "fuzzy" => Ok(Filter::Fuzzy),
            | _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Widget {
    Input {
        y: i32,
        filter: Filter,
        label: String,
        content: String,
        selectable: bool,
        hidden: bool,
        id: usize,
    },
    Text {
        y: i32,
        content: String,
        selectable: bool,
        show: bool,
        id: usize,
    },
}
