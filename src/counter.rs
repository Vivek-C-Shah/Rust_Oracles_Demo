pub struct Counter {
    pub prices: Vec<f64>,
    pub data: Vec<String>,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            prices: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn add_price(&mut self, price: f64, data: String) {
        self.prices.push(price);
        self.data.push(data)
    }

    pub fn calculate_average(&self) -> f64 {
        if self.prices.is_empty() {
            0.0
        } else {
            let sum: f64 = self.prices.iter().sum();
            sum / self.prices.len() as f64
        }
    }
}
