use megastore_search::search::{SearchIndex, Product};

#[test]
fn test_add_and_search() {
    let mut idx = SearchIndex::new();
    idx.add_product(Product { id: "1".into(), name: "Smartphone X".into(), brand: "BrandA".into(), category: "Eletrônicos".into()});
    idx.add_product(Product { id: "2".into(), name: "Camiseta Azul".into(), brand: "MarcaB".into(), category: "Vestuário".into()});
    let res = idx.search("smartphone");
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].id, "1");
}