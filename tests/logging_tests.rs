/*
* Bash and Blade
* @author: CaioSimioni
* @license: MIT
* NÃO ALTERE ESTE CABEÇALHO MANUALMENTE.
*/

// tests/logging_tests.rs
// Testes para o módulo de logging funcional do Bash and Blade

use bash_and_blade::utils::logging;
use std::fs;
use std::path::Path;

#[test]
fn test_setup_creates_log_file() {
    // Remove o arquivo de log se existir
    let log_path = "logs/game.logs";
    if Path::new(log_path).exists() {
        let _ = fs::remove_file(log_path);
    }
    // Executa o setup
    assert!(logging::setup().is_ok());
    // Verifica se o arquivo foi criado
    assert!(Path::new(log_path).exists());
}

#[test]
fn test_info_log_writes_to_file() {
    logging::setup().unwrap();
    let log_path = "logs/game.logs";
    let msg = "Mensagem de teste INFO";
    logging::info(msg);
    let content = fs::read_to_string(log_path).unwrap();
    assert!(content.contains(msg));
    assert!(content.contains("[INFO]"));
}

#[test]
fn test_warn_log_writes_to_file() {
    logging::setup().unwrap();
    let log_path = "logs/game.logs";
    let msg = "Mensagem de teste WARN";
    logging::warn(msg);
    let content = fs::read_to_string(log_path).unwrap();
    assert!(content.contains(msg));
    assert!(content.contains("[WARN]"));
}

#[test]
fn test_error_log_writes_to_file() {
    logging::setup().unwrap();
    let log_path = "logs/game.logs";
    let msg = "Mensagem de teste ERROR";
    logging::error(msg);
    let content = fs::read_to_string(log_path).unwrap();
    assert!(content.contains(msg));
    assert!(content.contains("[ERROR]"));
}
