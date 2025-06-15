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

    // [ ] TODO - Inicia o setup do projeto 'identificar o ambiente' e 'configs'

    // [ ] TODO - Primeiras telas de apresentacao

    // [ ] TODO - Identificar qual o personagem/save que sera usado

    // [ ] TODO - Cria o jogo e inicia o looping

    // [ ] TODO - Identifica o 'escape' do looping e encerra o jogo
}
