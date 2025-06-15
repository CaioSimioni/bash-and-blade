/*
* Bash and Blade
* @author: CaioSimioni
* @license: MIT
* NÃO ALTERE ESTE CABEÇALHO MANUALMENTE.
*/

extern crate bash_and_blade;

use bash_and_blade::utils::logging;

/*
* Main function that initializes the game.
*/
fn main() {
    // [x] TODO - Inicia o logging
    if let Err(e) = logging::setup() {
        eprintln!("Erro ao inicializar o logger: {}", e);
        std::process::exit(1);
    }

    logging::info("Bash and Blade - Iniciando o jogo");
    logging::warn("Este é um aviso de teste");
    logging::error("Este é um erro de teste");

    // [ ] TODO - Inicia o setup do projeto 'identificar o ambiente' e 'configs'

    // [ ] TODO - Primeiras telas de apresentacao

    // [ ] TODO - Identificar qual o personagem/save que sera usado

    // [ ] TODO - Cria o jogo e inicia o looping

    // [ ] TODO - Identifica o 'escape' do looping e encerra o jogo
}
