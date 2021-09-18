use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn meu_chute() -> u32 {
  let mut chute = String::new();
  io::stdin()
    .read_line(&mut chute)
    .expect("Deu erro no input");

  println!("Você chutou: {}", chute);

  let chute: u32 = match chute.trim().parse() {
    Ok(num) => num,
    Err(_) => {
      println!("você precisa digitar um número");
      0
    }
  };

  chute
}

fn gerar_numero_secreto() -> u32 {
  let numero_secreto: u32 = rand::thread_rng().gen_range(1, 11);

  numero_secreto
}

fn main() {
  println!("Adivinhe o número!");

  let numero_secreto = gerar_numero_secreto();

  println!("Por favor insira um número.");

  loop {
    let chute = meu_chute();
    match chute.cmp(&numero_secreto) {
      Ordering::Greater => println!("Valor muito alto!"),
      Ordering::Less => println!("Valor muito baixo"),
      Ordering::Equal => {
        println!("Mandou bem, acertou!! :)");
        break;
      }
    }
  }
}
