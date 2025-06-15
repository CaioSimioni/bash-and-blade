# **Bash & Blade** ⚔️⌨️  

*Um RPG de terminal escrito em Rust*  

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)  
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)](https://www.rust-lang.org/)  
[![Build Status](https://img.shields.io/github/actions/workflow/status/CaioSimioni/bash-and-blade/ci.yml?branch=master)](https://github.com/CaioSimioni/bash-and-blade/actions)

---

## 🎮 **Descrição**  

**Bash & Blade** é um RPG de texto jogável diretamente no terminal, onde você explora masmorras, resolve enigmas e enfrenta criaturas usando apenas o terminal. Feito em Rust, o jogo combina:  

- **Narrativa imersiva** (estilo *livro-jogo*).  
- **Combate baseado em turnos**.  
- **Desafios lógicos e interação via CLI**.  

---

## 🚀 **Objetivos**  

- [ ] Criar um framework de telas de terminal
- [ ] Criar as primeiras telas
- [ ] Dar contexto ao jogo. (Mapas, Itens, Personagens, etc.)

---

## ⚙️ **Como instalar o Rust**

Se ainda não possui o Rust instalado, execute:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Após a instalação, reinicie o terminal e verifique:

```bash
rustc --version
cargo --version
```

---

## 💻 **Comandos para Desenvolver, Testar e Rodar**

### Baixar o projeto

```bash
git clone https://github.com/CaioSimioni/bash-and-blade.git
cd bash-and-blade
```

### Compilar e rodar o jogo

```bash
cargo run --release
```

### Rodar os testes automatizados

```bash
cargo test
```

### Gerar documentação

```bash
cargo doc --open
```

### Outras opções úteis

- **Checar o código sem compilar:**

  ```bash
  cargo check
  ```

- **Formatar o código:**  

  ```bash
  cargo fmt
  ```

- **Analisar possíveis erros/lints:**  

  ```bash
  cargo clippy
  ```

---

## 📂 **Estrutura do Projeto**

```bash
.
├── src/                  # Código-fonte Rust
│   ├── main.rs           # Ponto de entrada
│   ├── game/             # Lógica do jogo (módulos separados)
│   └── tests/            # Testes automatizados
├── docs/                 # Documentação técnica
├── assets/               # Arquivos de texto (história, diálogos)
└── Cargo.toml            # Dependências e metadados
```

---

## 📦 **Dependências**

Adicione as dependências ao arquivo `Cargo.toml`:

Depois, baixe e instale as dependências com:

```bash
cargo build
```

---

## 🔧 **Como Contribuir**  

1. **Clone o repositório**:  

   ```bash
   git clone https://github.com/seuuser/bash-and-blade.git
   ```

2. **Compile e execute**:  

   ```bash
   cargo run --release
   ```

3. **Siga as diretrizes** em [CONTRIBUTING.md](CONTRIBUTING.md).  

---

## 📜 **Licença**  

Distribuído sob a licença **MIT**. Veja [LICENSE](LICENSE) para detalhes.
