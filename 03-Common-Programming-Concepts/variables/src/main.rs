fn main() {
    // Compile error:
    // let x = 5;
    // println!("X é {x}");
    // x = 6;
    // println!("X é {x}");
    //
    // OK:
    let mut x = 5;
    println!("X é {x}");
    x = 6;
    println!("X é {x}");

    // Constantes devem ser declaradas
    // com o comando const e devem sempre
    // ter o tipo anotado explicitamente.
    // Por convenção, utiliza-se caixa alta.
    //
    const TRES_HORAS_EM_SEGUNDOS: u32 = 60 * 60 * 3;
    println!("Três horas = {TRES_HORAS_EM_SEGUNDOS} segundos");
}
