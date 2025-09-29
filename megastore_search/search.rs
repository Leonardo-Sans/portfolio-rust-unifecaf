use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub brand: String,
    pub category: String,
}

pub struct SearchIndex {
    pub index: HashMap<String, Vec<String>>,
    pub products: HashMap<String, Product>,
}

impl SearchIndex {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            products: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, p: Product) {
        let id = p.id.clone();
        for token in tokenize(&p.name).into_iter().chain(tokenize(&p.brand)).chain(tokenize(&p.category)) {
            self.index.entry(token).or_default().push(id.clone());
        }
        self.products.insert(id.clone(), p);
    }

    pub fn search(&self, query: &str) -> Vec<&Product> {
        let mut results: Vec<&Product> = Vec::new();
        let tokens = tokenize(query);
        let mut seen = std::collections::HashSet::new();
        for t in tokens {
            if let Some(ids) = self.index.get(&t) {
                for id in ids {
                    if seen.insert(id) {
                        if let Some(p) = self.products.get(id) {
                            results.push(p);
                        }
                    }
                }
            }
        }
        results
    }
}

fn tokenize(s: &str) -> Vec<String> {
    s.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_string())
        .collect()
}