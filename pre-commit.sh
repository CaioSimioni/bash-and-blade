#!/bin/bash
# ============================================================
# pre-commit.sh - Pre-commit hook para Rust (cargo)
# Executa testes, build e lints antes do commit
# ============================================================
set -e

main() {
    echo ""
    echo "=============================================="
    echo " 🚀  Iniciando pre-commit para projeto Rust"
    echo "=============================================="
    echo ""

    echo -n "🧪  Executando testes... "
    if cargo test --quiet > /dev/null 2>&1; then
        echo "OK"
    else
        echo "FALHOU"
        echo "Saída do cargo test:"
        cargo test
        exit 1
    fi

    echo -n "🔨  Compilando o projeto... "
    if cargo build --release --quiet > /dev/null 2>&1; then
        echo "OK"
    else
        echo "FALHOU"
        echo "Saída do cargo build:"
        cargo build --release
        exit 1
    fi

    echo -n "📝  Verificando formatação... "
    if cargo fmt -- --check --quiet > /dev/null 2>&1; then
        echo "OK"
    else
        echo "FALHOU"
        echo "A formatação do código está incorreta."
        echo "Por favor, execute 'cargo fmt' manualmente para corrigir."
        cargo fmt -- --check
        exit 1
    fi

    echo -n "🔍  Executando lints... "
    if cargo clippy --quiet -- -D warnings > /dev/null 2>&1; then
        echo "OK"
    else
        echo "FALHOU"
        echo "Saída do cargo clippy:"
        cargo clippy -- -D warnings
        exit 1
    fi

    echo ""
    echo "✅  Pre-commit finalizado com sucesso!"
    echo ""
}

main "$@"
