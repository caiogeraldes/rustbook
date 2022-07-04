fn main() {
    // Se o código dissesse:
    // let guess = "42".parse().expect("Não é número não!");
    // error[E0282]: type annotations needed
    // --> src/main.rs:2:9
    // |
    // 2 |     let guess = "42".parse().expect("Not a number!");
    // |         ^^^^^ consider giving `guess` a type
    //
    // For more information about this error, try `rustc --explain E0282`.
    // error: could not compile `no_type_annotations` due to previous error
    //
    // Portanto, o correto é:
    let guess: u32 = "42".parse().expect("Não é número não!");

    println!("{guess}");

    // O código abaixo causaria um overflow:
    //
    // let mut x: u8 = 255;
    // println!("X é {x}");
    //
    // x = x + 1;
    // println!("X é {x}");
    //
    // Se a compilação for em --release, o compilador
    // não panicaria e ocorreria realmente um two's
    // complement error: x seria 0.
    // Usar esse comportamento propositalmente não é
    // boa prática.
}
