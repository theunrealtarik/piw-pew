pub struct Window {
    pub name: String,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            name: String::from("PPS"),
        }
    }
}
