#!/bin/bash
# ============================================================
# headers.sh - Script para atualizar cabeçalhos de arquivos Rust
# Este script verifica se os arquivos Rust possuem um cabeçalho específico e o atualiza ou adiciona conforme necessário.
# ============================================================

HEADER="/*\n* Bash and Blade\n* @author: CaioSimioni\n* @license: MIT\n* NÃO ALTERE ESTE CABEÇALHO MANUALMENTE.\n*/"

update_headers() {
    echo "Alterando os seguintes arquivos:"
    find ./src ./tests -type f -name "*.rs" | while read FILE; do
        if grep -q "NÃO ALTERE ESTE CABEÇALHO MANUALMENTE" "$FILE"; then
            # Atualiza apenas o cabeçalho existente
            awk -v header="$HEADER" 'BEGIN{printed=0} \
                NR==1 && $0 ~ /\/\*/ {print header; printed=1; next} \
                NR<=5 && $0 ~ /NÃO ALTERE ESTE CABEÇALHO MANUALMENTE/ {next} \
                {if(NR==1 && !printed) print header; print $0}' "$FILE" > "$FILE.tmp" && mv "$FILE.tmp" "$FILE"
            echo "Cabeçalho atualizado: $FILE"
        else
            # Insere o cabeçalho no topo
            echo -e "$HEADER\n\n$(cat "$FILE")" > "$FILE"
            echo "Cabeçalho adicionado: $FILE"
        fi
    done
}

main() {
    update_headers
    echo "Processo concluído."
}

main
