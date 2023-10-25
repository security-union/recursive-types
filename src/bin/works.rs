enum Matryoshka {
    Inner(i32, Box<Matryoshka>),
    Nil,
}

fn main() {
    let my_matryoshka = Matryoshka::Inner(
        0,
        Box::new(Matryoshka::Inner(
            1,
            Box::new(Matryoshka::Inner(
                2,
                Box::new(Matryoshka::Inner(3, Box::new(Matryoshka::Nil))),
            )),
        )),
    );

    println!("Matryoshka created!");
}
