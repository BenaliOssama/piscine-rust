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
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, store: &Store, name: String) {
        if let Some(product) = store.products.iter().find(|p| p.0 == name) {
            self.items.push((name, product.1));
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut values: Vec<f32> = self.items
            .iter()
            .map(|(_, v)| *v)
            .collect();
        let discount = self.items.len() / 3;
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let payable: f32 = values[discount..].iter().sum();
        let total: f32 = values.iter().sum();

        let ratio: f32 = (payable * 100.0) / total;

        self.receipt = values
            .iter()
            .map(|val| round2((val * ratio) / 100.0))
            .collect();

        self.receipt.clone()
    }
}

fn round2(x: f32) -> f32 {
    (x * 100.0).round() / 100.0
}
