use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivinhe o Número");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Por favor, digite seu palpite:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Ops... ocorreu um erro ao ler a linha.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você chutou o número: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("O número está muito abaixo!"),
            Ordering::Greater => println!("Muito alto...!"),
            Ordering::Equal => {
                println!("Wow! Você acertou!!!");
                break;
            }
        }
    }
}
