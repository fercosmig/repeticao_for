fn main()
{
    for i in 0..10
    {
        print!("{} ", i);
    }
    println!(";");

    let range = 0..20;

    for i in range
    {
        print!("{} ", i);
    }
    println!(";");

    let animais = vec!["Coelho", "Gato", "Macaco", "Cachorro"];

    for a in animais
    {
        print!("{} ", a);
    }
    println!(";");
}
