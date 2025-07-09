use code::set5::frogs_mosquitos::frog_mosquitos;

fn main() {
    let frogs = vec![(10,2),(15,0),(6,1),(0,1)];
    let mosquitos = vec![
        (110,10),
        (1,1),
        (6,0),
        (15,10),
        (14,100),
        (12,2)
    ];

    let v = frog_mosquitos(&frogs, &mosquitos);
    println!("{v:#?}");
}