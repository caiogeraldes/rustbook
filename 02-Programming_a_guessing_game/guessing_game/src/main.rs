use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivinhe o número!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Dê seu chute!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Eita, algo deu errado!");

    println!("Seu número escolhido foi: {guess}");

    // match guess.cmp(&secret_number) {
    //     Ordering::Less => println!("Baixo demais!"),
    //     Ordering::Greater => println!("Grande demais!"),
    //     Ordering::Equal => println!("Acertou!"),
    // }

    println!("O número secreto era: {secret_number}");
}
