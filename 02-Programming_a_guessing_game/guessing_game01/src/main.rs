//
// Importando uma biblioteca que permita
// ler interações passadas via IO.
//
use std::io;

fn main() {
    println!("Adivinhe o número!");

    println!("Dê um chute.");

    //
    // Cria uma variável mutável de nome
    // guess, cujo valor é o resultado da
    // função String::new(), uma função que
    // retorna uma nova instância de uma
    // variável do tipo String.
    //
    // Os dois pontos (::) indicam que new
    // é uma função associada ao tipo String.
    // Uma função associada é uma função
    // implementada em um tipo.
    //
    // String::new() cria uma string vazia.
    let mut guess = String::new();

    // Se não tivéssimos importado std::io,
    // para usar a função stdin(), deveríamos
    // chamar:
    //
    // std::io::stdin()
    //
    // A função stdin de io retorna uma instância
    // de std::io::Stdin, um tipo que representa
    // uma manopla para o standard input do terminal.
    //
    io::stdin()
        // O método .read_line(&mut guess) recebe o input
        // do usuário e a guarda na variável guess.
        // .read_line recebe o que o usuário passa como
        // imput e cola ao final de uma string.
        // Isso significa que, fosse o código:
        //
        //  let mut guess = String::from("a");
        //
        //  E o usuário entrasse com o valor 1, o valor
        //  de guess reportado pelo print seria "a1".
        //  O argumento de .read_line precisa ser mutável,
        //  caso o contrário ele não pode realizar sua
        //  função.
        //
        //  O & diz que guess passado para read_line é uma
        //  referência, o que permite que múltiplas partes
        //  do programa possam acessar esse dado sem
        //  precisar copiá-lo múltiplas vezes na memória.
        //  Esse código não funcionaria se a linha lesse:
        //
        //  .read_line(&guess)
        //
        //  ou
        //
        //  .read_line(guess)
        //
        .read_line(&mut guess)
        //
        // .read_line retorna um objeto do tipo Result.
        // Result é uma enumeração / enum, um tipo que
        // pode se realizar em múltiplos estados diferentes.
        // Cada tipo é chamado de variante.
        // As duas variantes de Result são Ok e Err.
        // Ok indica que a operação foi completada com
        // sucesso e dentro dela está o valor gerado.
        // Err contém a informação de como ou porque a
        // operação falhou.
        //
        // Result tem um método expect que imprime seu
        // argumento caso o valor de Result seja do tipo
        // Err, caso o contrário, retorna os bytes do input
        // guardados em Ok.
        //
        .expect("Não consegui ler a linha!");

    // Para imprimir o valor, utiliza-se a sintaxe abaixo:
    //
    println!("Você chutou: {guess}");
}
