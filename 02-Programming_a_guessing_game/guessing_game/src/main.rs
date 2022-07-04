use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivinhe o número!");

    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    let mut tries: u8 = 0;

    // Início do loop
    //
    loop {
        tries += 1;
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
        // u8 para acomodar a forma numérica da String
        // armazenada em guess. Como guess é mutável,
        // podemos utilizar o mesmo nome de variável
        // para a nossa u8.
        //
        // Para  isso, é necessário realizar algumas
        // operações sobre guess (String):
        // - trim(): elimina espaços em branco no começo
        //      ou fim da linha do standard input.
        // - parse(): converte uma string em outro tipo
        //      que pode ser declarado no ato de instanciar
        //      uma variável de tipo X. Assim, quando
        //      passamos u8 no let-statement, parse()
        //      sabe que deve converter a string para
        //      unsigned integer de 8 bits.
        //
        // Ao invés de utilizar .expect("mensagem") após
        // parse para lidar com erros, o que faria o
        // programa romper após o erro, o que pareceria com:
        //
        // let guess: u8 = guess.trim().parse().expect("a!")
        //
        // , utiliza-se um match statement que avalia se o
        // objeto Result retornado por parse() é da variante
        // Ok ou Err. Caso Ok, retorna-se o num contido em
        // Ok, caso o contrário, se imprime uma mensagem de
        // erro e se comanda a continuidade do loop.
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, um número, seu cavalo!");
                continue;
            }
        };

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

    println!("Você demorou {tries} tentativas para acertar!");
}
