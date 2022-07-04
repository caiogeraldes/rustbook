// Este script foi criado usando a aplicação cargo.
//
// $ cargo new hello_cargo
//
// Este comando cria uma estrutura de arquivos:
// hello_cargo --- src --- main.rs
//              |- Cargo.toml
//
// Ao rodar:
//
// $ cargo run
//
// Cria-se uma nova pasta "target" e um arquivo Cargo.lock:
//
// hello_cargo --- src --- main.rs
//              |- target --- CACHEDIR.TAG
//              |          |- debug/...
//              |- Cargo.lock
//              |- Cargo.toml
//
// Em ./target/debug/ está o binário executável hello_cargo:
//
// $ ./target/debug/hello_cargo
// # Olar, Cargo!
// #
//
// Essa mesma estrutura teria sido criada com o comando:
//
// $ cargo build
//
// Para realizar optmizações e gerar o executável para distribuição,
// deve-se rodar:
//
// $ cargo build --release
//
// Este comando adiciona à pasta target uma pasta release
//
// hello_cargo --- src --- main.rs
//              |- target --- CACHEDIR.TAG
//              |          |- debug/...
//              |          |- release/...
//              |- Cargo.lock
//              |- Cargo.toml
//
fn main() {
    println!("Olar, Cargo!");
}
