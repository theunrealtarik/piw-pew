pub struct Health {
    value: f32,
    threshold: f32,
    percentage: f32,
}

#[allow(dead_code)]
impl Health {
    pub fn new(base: f32) -> Self {
        Self {
            value: base,
            threshold: base,
            percentage: 100.0,
        }
    }

    pub fn damage(&mut self, amount: f32) {
        self.value = self.value - amount;
        self.percentage = self.value / self.threshold * 100.0;
    }

    pub fn heal(&mut self, amout: f32) {
        self.value = self.value + amout;
        self.percentage = self.value / self.threshold * 100.0;
    }

    pub fn set(&mut self, hp: f32) {
        self.value = hp;
    }
}
