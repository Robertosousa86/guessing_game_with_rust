use std::io;

fn main() {
    println!("Adivinhe o Número");

    println!("Por favor, digite seu palpite");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Ops... ocorreu um erro ao ler a linha.");

    println!("Você chutou o número: {}", guess);
}
