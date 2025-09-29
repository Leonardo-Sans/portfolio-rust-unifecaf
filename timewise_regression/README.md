# Portfólio em Rust: Regressão Linear Pura para Séries Temporais

**Aluno:** Leonardo dos santos da silva  
**Instituição:** UNIFECAF

## Descrição
Implementação "pura" de regressão linear simples em Rust sem usar crates numéricos externos.
Fornece funções para ajustar, prever e calcular MSE e R².

## Como executar
1. Instale Rust (https://rustup.rs)
2. No diretório do projeto:
   ```bash
   cargo test
   cargo build
   ```
3. Exemplos de uso (na própria biblioteca ou em um binário adicionado).

## Arquivos
- `src/linreg.rs`: Implementação da regressão.
- `tests/linreg_test.rs`: Teste unitário que verifica fit, predict, MSE e R².