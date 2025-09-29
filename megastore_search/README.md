# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

**Aluno:** Leonardo dos santos da silva  
**Instituição:** UNIFECAF

## Descrição
Projeto de exemplo que implementa um índice em memória usando `HashMap` para mapear termos de busca a IDs de produtos.
Proposta didática para o estudo de caso da disciplina.

## Estrutura
- `src/main.rs`: CLI de exemplo que carrega `data/products.csv` e permite consultas interativas.
- `src/search.rs`: Implementação do índice e tokenização.
- `tests/`: Teste unitário básico.
- `data/products.csv`: CSV de exemplo (não incluído aqui).

## Como executar
1. Instale Rust (https://rustup.rs)
2. No diretório do projeto:
   ```bash
   cargo build
   cargo run --release
   ```
3. Coloque um CSV em `data/products.csv` com linhas `id,name,brand,category`.

## Testes
Rode:
```bash
cargo test
```

## Observações técnicas
- A tokenização aqui é simples; para produção, considerar normalização, lematização, stemming, e controle de stop-words.
- Para escalabilidade considere persistir índices (sled, rocksdb) e usar sharding/particionamento.