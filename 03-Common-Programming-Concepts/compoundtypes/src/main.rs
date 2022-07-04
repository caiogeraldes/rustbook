fn main() {
    // Tuplas:
    //  - imutáveis em tamanho,
    //  - tipos independentes entre elementos,
    //  - elementos chamados por .

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("1º item da tupla: {}", tup.0);
    println!("2º item da tupla: {}", tup.1);
    println!("3º item da tupla: {}", tup.2);

    // Arrays:
    //  - imutáveis em tamanho,
    //  - tipo único,
    //  - elementos chamados por [x],
    //  - alocados em stack.
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("1º item da array: {}", a[0]);
    println!("2º item da array: {}", a[1]);
    println!("3º item da array: {}", a[2]);
    println!("4º item da array: {}", a[3]);
    println!("5º item da array: {}", a[4]);

    let meses = [
        "Janeiro",
        "Fevereiro",
        "Março",
        "Abril",
        "Maio",
        "Junho",
        "Julho",
        "Agosto",
        "Setembro",
        "Outubro",
        "Novembro",
        "Dezembro",
    ];

    println!("Primeiro mês é: {}", meses[0]);
    println!("Sexto mês é: {}", meses[5]);
}
