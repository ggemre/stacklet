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

// #[derive(Debug, Clone)]
// pub struct Model(pub Vec<Widget>);

// impl Model {
//     pub fn get_by_y(&self, y: i32) -> Option<&Widget> {
//         self.iter.find(|&widget| {
//             match widget {
//                 Widget::Input { y: widget_y, .. } | Widget::Text { y: widget_y, .. } => *widget_y == y,
//             }
//         })
//     }

//     pub fn get(&self, index: usize) -> Option<&Widget> {
//         self.0.get(index)
//     }

//     pub fn get_mut(&self, index: usize) -> Option<&Widget> {
//         self.0.get_mut(index).as_deref()
//     }
// }

// impl<'a> IntoIterator for &'a Model {
//     type Item = &'a Widget;
//     type IntoIter = std::slice::Iter<'a, Widget>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.0.iter()
//     }
// }
