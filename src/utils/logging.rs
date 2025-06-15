/*
* Bash and Blade
* @author: CaioSimioni
* @license: MIT
* NÃO ALTERE ESTE CABEÇALHO MANUALMENTE.
*/

//! Sistema de logging funcional para o projeto Bash and Blade.
//!
//! # Como usar
//!
//! 1. Importe o módulo:
//!    use bash_and_blade::utils::logging;
//!
//! 2. Inicialize o logger no início do seu programa:
//!    logging::setup().expect("Erro ao inicializar o logger");
//!
//! 3. Utilize as funções de log conforme necessário:
//!    logging::info("Mensagem informativa");
//!    logging::warn("Mensagem de aviso");
//!    logging::error("Mensagem de erro");
//!
//! # Detalhes Técnicos
//!
//! - O arquivo de log é salvo em `logs/game.logs` na raiz do projeto.
//! - O logger cria automaticamente a pasta `logs/` se ela não existir.
//! - O formato de cada linha de log é: `[YYYY-MM-DD HH:MM:SS][NÍVEL] Mensagem`
//! - Os níveis disponíveis são: INFO, WARN e ERROR.
//! - As mensagens não são exibidas no terminal, apenas gravadas no arquivo.
//! - O logger só grava mensagens após a chamada de `setup()`.
//! - O logger é seguro para chamadas múltiplas de setup (usa Once).
//! - Se o setup falhar, nenhuma mensagem será gravada.
//!
//! # Exemplo Completo
//!
//! ```rust
//! use bash_and_blade::utils::logging;
//! logging::setup().expect("Erro ao inicializar o logger");
//! logging::info("Jogo iniciado");
//! logging::warn("Aviso de teste");
//! logging::error("Erro de teste");
//! ```

use chrono::Local;
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::path::Path;
use std::sync::Once;

static INIT: Once = Once::new();
static mut LOG_READY: bool = false;
const LOG_PATH: &str = "logs/game.logs";

/// Inicializa o sistema de logging, criando diretórios e testando escrita.
///
/// # Exemplo
///
/// ```rust,no_run
/// use bash_and_blade::utils::logging;
/// assert!(logging::setup().is_ok());
/// ```
pub fn setup() -> Result<(), String> {
    let mut result = Ok(());
    INIT.call_once(|| {
        if let Some(parent) = Path::new(LOG_PATH).parent() {
            if !parent.exists() {
                if let Err(e) = create_dir_all(parent) {
                    result = Err(format!("Erro ao criar diretório de logs: {}", e));
                    return;
                }
            }
        }
        let now = Local::now().format("%Y-%m-%d %H:%M:%S");
        let test_message = format!(
            "[{}][INFO] Logger inicializado com sucesso.
",
            now
        );
        match OpenOptions::new().create(true).append(true).open(LOG_PATH) {
            Ok(mut file) => {
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
        unsafe {
            LOG_READY = true;
        }
    });
    result
}

fn write_log(level: &str, message: &str) {
    unsafe {
        if !LOG_READY {
            return;
        }
    }
    let now = Local::now().format("%Y-%m-%d %H:%M:%S");
    let formatted = format!(
        "[{}][{}] {}
",
        now, level, message
    );
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
