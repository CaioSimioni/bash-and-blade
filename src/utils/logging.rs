/*
* Bash and Blade
* @author: CaioSimioni
* @license: MIT
* NÃO ALTERE ESTE CABEÇALHO MANUALMENTE.
*/

//! Sistema de logging funcional para o projeto Bash and Blade.
//!
//! **Como usar**
//!
//! 1. Importe o módulo:
//!    use bash_and_blade::utils::logging;
//!
//! 2. Inicialize o logging no início do seu programa:
//!    logging::setup().expect("Erro ao inicializar o logging");
//!
//! 3. Utilize as funções de log conforme necessário:
//!    logging::info("Mensagem informativa");
//!    logging::warn("Mensagem de aviso");
//!    logging::error("Mensagem de erro");
//!
//! **Detalhes Técnicos**
//!
//! - O arquivo de log é salvo em `logs/game.logs` na raiz do projeto.
//! - O logging cria automaticamente a pasta `logs/` se ela não existir.
//! - O formato de cada linha de log é: `[YYYY-MM-DD HH:MM:SS][NÍVEL] Mensagem`
//! - Os níveis disponíveis são: INFO, WARN e ERROR.
//! - As mensagens não são exibidas no terminal, apenas gravadas no arquivo.
//! - O logging só grava mensagens após a chamada de `setup()`.
//! - O logging é seguro para chamadas múltiplas de setup (usa Once).
//! - Se o setup falhar, nenhuma mensagem será gravada.
//!
//! ```rust
//! use bash_and_blade::utils::logging;
//! logging::setup().expect("Erro ao inicializar o logging");
//! logging::info("Jogo iniciado");
//! logging::warn("Aviso de teste");
//! logging::error("Erro de teste");
//! ```

use chrono::Local;
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::path::Path;
use std::sync::Once;

// Variáveis estáticas para garantir inicialização única do logging
static INIT: Once = Once::new();
static mut LOG_READY: bool = false;
const LOG_PATH: &str = "logs/game.logs";

/// Inicializa o sistema de logging, criando diretórios e testando escrita.
///
/// Retorna `Ok(())` se o logging foi inicializado com sucesso.
///
/// Caso contrário, retorna `Err(String)` com a mensagem de erro.
///
/// ```rust,no_run
/// use bash_and_blade::utils::logging;
/// assert!(logging::setup().is_ok());
/// ```
pub fn setup() -> Result<(), String> {
    let mut result: Result<(), String> = Ok(());

    // Garante que o setup só será executado uma vez, mesmo que chamado várias vezes
    INIT.call_once(|| {
        // Verifica se o diretório do arquivo de log existe, se não existir, cria
        if let Some(parent) = Path::new(LOG_PATH).parent() {
            if !parent.exists() {
                if let Err(e) = create_dir_all(parent) {
                    result = Err(format!("Erro ao criar diretório de logs: {}", e));
                    return;
                }
            }
        }

        // Prepara uma mensagem de teste para registrar que o logging foi inicializado
        let now: chrono::format::DelayedFormat<chrono::format::StrftimeItems<'_>> =
            Local::now().format("%Y-%m-%d %H:%M:%S");
        let test_message = format!("[{}][INFO] Logging inicializado com sucesso.", now);

        // Tenta abrir (ou criar) o arquivo de log e escrever a mensagem de teste
        match OpenOptions::new().create(true).append(true).open(LOG_PATH) {
            Ok(mut file) => {
                // Adiciona \n ao final da mensagem para garantir quebra de linha
                let test_message = format!("{}\n", test_message);
                if let Err(e) = file.write_all(test_message.as_bytes()) {
                    result = Err(format!("Erro ao escrever no arquivo de log: {}", e));
                    return;
                }
            }
            Err(e) => {
                result = Err(format!("Erro ao abrir arquivo de log: {}", e));
                return;
            }
        }

        // Marca que o logging está pronto para uso
        unsafe {
            LOG_READY = true;
        }
    });

    result
}

// Função interna para escrever uma mensagem no arquivo de log
fn write_log(level: &str, message: &str) {
    // Só grava se o logging estiver pronto
    unsafe {
        if !LOG_READY {
            return;
        }
    }

    // Monta a mensagem no formato desejado
    let now: chrono::format::DelayedFormat<chrono::format::StrftimeItems<'_>> =
        Local::now().format("%Y-%m-%d %H:%M:%S");
    // Adiciona \n ao final para garantir quebra de linha
    let formatted: String = format!("[{}][{}] {}\n", now, level, message);

    // Abre o arquivo de log e escreve a mensagem
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(LOG_PATH) {
        let _ = file.write_all(formatted.as_bytes());
    }
}

/// Loga uma mensagem de nível INFO.
///
/// # Exemplo
///
/// ```rust,no_run
/// use bash_and_blade::utils::logging;
/// logging::setup().unwrap();
/// logging::info("Mensagem informativa");
/// ```
pub fn info(message: &str) {
    write_log("INFO", message);
}

/// Loga uma mensagem de nível WARN.
///
/// # Exemplo
///
/// ```rust,no_run
/// use bash_and_blade::utils::logging;
/// logging::setup().unwrap();
/// logging::warn("Mensagem de aviso");
/// ```
pub fn warn(message: &str) {
    write_log("WARN", message);
}

/// Loga uma mensagem de nível ERROR.
///
/// # Exemplo
///
/// ```rust,no_run
/// use bash_and_blade::utils::logging;
/// logging::setup().unwrap();
/// logging::error("Mensagem de erro");
/// ```
pub fn error(message: &str) {
    write_log("ERROR", message);
}
