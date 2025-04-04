fn main() {
    let cat: (&str, f64) = ("Furry McFurson", 3.5);

    // Destructure the `cat` tuple in one statement
    let (name, age) = cat;

    println!("{name} is {age} years old");
}