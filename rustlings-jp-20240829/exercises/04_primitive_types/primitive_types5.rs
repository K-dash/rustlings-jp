fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: `cat`タプルを以下のprintln!が適切に動くように分解してください。
    // let /* your pattern here */ = cat;
    let name = cat.0;
    let age = cat.1;

    println!("{name} is {age} years old");
}
