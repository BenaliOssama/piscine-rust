#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, s: &Store, element: String) {
        if let Some(item) = s.products.iter().find(|pr| pr.0 == element) {
            self.items.push((ele, item.1));
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut values: Vec<f32> = self.items
            .iter()
            .map(|(_, &p)| p)
            .collect();
        let freebies = self.items.len() / 3;

        let mut sorted = values.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let full: f32 = sorted.iter().sum();
        let reduced: f32 = sorted.iter().skip(freebies).sum();
        let ratio = reduced / full;

        self.receipt = sorted
            .iter()
            .map(|&p| round_two(p * ratio))
            .collect();
        self.receipt.clone()
    }
}

fn round_two(nbr: f32) -> f32 {
    (nbr * 100.0).round() / 100.0
}
