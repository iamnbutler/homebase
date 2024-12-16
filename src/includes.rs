pub struct Includes {
    pub styles: Vec<&'static str>,
}

pub fn includes() -> Includes {
    Includes {
        styles: vec!["global.css"],
    }
}
