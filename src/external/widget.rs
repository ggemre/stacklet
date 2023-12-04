#[derive(Debug)]
pub enum Widget {
    Input { 
        y: usize,
        max_width: usize,
        filter: bool,
        label: String,
        placeholder: String,
        content: String,
    },
    Text { 
        y: usize,
        content: String,
    },
}

