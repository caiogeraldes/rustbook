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
fn main() {
    println!("Olar, Cargo!");
}
