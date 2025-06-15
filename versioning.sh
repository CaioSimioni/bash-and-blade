#!/bin/bash

load_env() {
    if [ -f .env ]; then
        source .env
    else
        echo ".env file not found!"
        exit 1
    fi
}

get_version() {
    VERSION=$VERSION
}

confirm_version() {
    echo "Versão atual no .env: $VERSION"
    read -p "Tem certeza que deseja aplicar essa versão? (s/n): " CONFIRM
    if [[ "$CONFIRM" != "s" ]]; then
        echo "Abortado."
        exit 0
    fi
}

update_headers() {
    HEADER="/*\n* Bash and Blade\n* @version: v$VERSION\n* @author: CaioSimioni\n* @License: MIT\n*/"
    echo "Alterando os seguintes arquivos:"
    find ./src -type f -name "*.rs" | while read FILE; do
        if grep -q "\* @version:" "$FILE"; then
            # Atualiza apenas o texto da linha que contém * @version:
            sed -i -E "s#(\* @version: ).*#\1v$VERSION#" "$FILE"
            echo "Versão atualizada: $FILE"
        else
            # Insere o cabeçalho no topo
            echo -e "$HEADER\n\n$(cat "$FILE")" > "$FILE"
            echo "Cabeçalho adicionado: $FILE"
        fi
    done
}

main() {
    load_env
    get_version
    confirm_version
    update_headers
    echo "Processo concluído."
}

main
