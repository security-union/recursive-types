enum Matryoshka {
    Inner(i32, Matryoshka),
    Nil,
}

fn main() {
    let my_matryoshka = Matryoshka::Inner(42, Matryoshka::Inner(43, Matryoshka::Nil));
    println!("Matryoshka created!");
}
