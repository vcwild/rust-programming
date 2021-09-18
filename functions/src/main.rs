fn main() {
    let minha_string = String::from("minha string");

    let primeirinha = primeira_palavra(&minha_string);

    println!("a primeirinha é: {}", primeirinha);
}

fn primeira_palavra(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s
}
