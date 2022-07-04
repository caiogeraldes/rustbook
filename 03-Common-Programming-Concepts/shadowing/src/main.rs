fn main() {
    let x = 5;
    let mut y = 5;

    println!("X é {x}");
    println!("Y é {y}");

    // Shadowing é o ato de declarar uma variável
    // com o mesmo nome de uma anterior. Neste caso,
    // a segunda variável toma para si o nome
    // da primeira até que o escopo acabe ou ela
    // mesma seja sombreada.
    let x = x + 1;
    y = y + 1;
    println!("X, agora, é {x}");
    println!("Y, agora, é {y}");

    {
        // Note-se que sem let, y permanecerá sendo
        // (y + y + 1) * 2 quando este escopo acabar.
        let x = x * 2;
        y = y * 2;
        println!("X, aqui dentro, é {x}");
        println!("Y, aqui dentro, é {y}");
    }

    println!("X, aqui fora, é {x}");
    println!("Y, aqui fora, é {y}");

    // Como com sombreamento se cria uma nova variável
    // com o mesmo nome de outra, pouco importa o tipo
    // dela no momento da primeira declaração.

    let spaces: String = String::from("   ");
    let spaces = spaces.len();

    // Erro:
    // let mut spaces: String = String::from("   ");
    // spaces = spaces.len();

    println!("A string de três espaços brancos tem {spaces} caracteres.");
}
