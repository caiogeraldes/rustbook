use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivinhe o número!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut tentativas = 0;

    // Início do loop
    //
    loop {
        tentativas += 1;
        println!("Dê seu chute!");

        // Criação de uma variável para receber
        // a string do terminal passada pelo
        // usuário.
        //
        let mut guess = String::new();

        // Leitura do valor passado pelo usuário e
        // checagem para erros.
        //
        io::stdin()
            .read_line(&mut guess)
            .expect("Eita, algo deu errado!");

        // É necessário criar uma variável do tipo
        // u32 para acomodar a forma numérica da String
        // armazenada em guess. Como guess é mutável,
        // podemos utilizar o mesmo nome de variável
        // para a nossa u32.
        let guess: u32 = guess.trim().parse().expect("Por favor, um número, cavalo!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Baixo demais!"),
            Ordering::Greater => println!("Grande demais!"),
            Ordering::Equal => {
                // Caso o usuário acerte, há a quebra
                // do loop, caso o contrário, o programa
                // pediria novamente um chute.
                println!("Acertou!");
                break;
            }
        }
    }

    println!("Você demorou {tentativas} tentativas para acertar!");
}
