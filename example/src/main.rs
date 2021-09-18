fn main() {
    let s1 = String::from("texto");

    let tamanho = strlen(&s1);

    println!("O tamanho de '{}' é {}.", s1, tamanho);
}

fn strlen(s: &String) -> usize {
    s.len()
}
