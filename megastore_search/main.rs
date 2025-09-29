use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Estrutura simples de produto
#[derive(Debug, Clone)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub brand: String,
    pub category: String,
}

// Índice em memória: termo -> lista de IDs
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
        // indexar por nome, marca e categoria (tokens simples)
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

// Le um CSV simples: id,name,brand,category
pub fn load_products_from_csv(path: &str) -> io::Result<Vec<Product>> {
    let mut out = Vec::new();
    let file = File::open(path)?;
    for line in io::BufReader::new(file).lines() {
        let l = line?;
        if l.trim().is_empty() { continue; }
        let parts: Vec<&str> = l.split(',').map(|p| p.trim()).collect();
        if parts.len() < 4 { continue; }
        out.push(Product {
            id: parts[0].to_string(),
            name: parts[1].to_string(),
            brand: parts[2].to_string(),
            category: parts[3].to_string(),
        });
    }
    Ok(out)
}

fn main() -> io::Result<()> {
    // Arquivo de exemplo: data/products.csv (forneça seu CSV)
    let sample = "data/products.csv";
    if Path::new(sample).exists() {
        let products = load_products_from_csv(sample)?;
        let mut idx = SearchIndex::new();
        for p in products {
            idx.add_product(p);
        }

        println!("Índice criado. Digite consultas (enter para sair):");
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let q = line?;
            if q.trim().is_empty() { break; }
            let res = idx.search(&q);
            println!("Resultados ({}):", res.len());
            for p in res.iter().take(20) {
                println!("- {} | {} | {}", p.id, p.name, p.brand);
            }
            println!("---");
        }
    } else {
        println!("Arquivo {} não encontrado. Coloque um CSV em data/products.csv com: id,name,brand,category", sample);
    }
    Ok(())
}