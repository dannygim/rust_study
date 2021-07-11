pub fn run() {
    let mut hello = String::from("Hello");

    println!("{}, world!", hello);
    println!("Lenth: {}", hello.len());

    hello.push_str(" World");
    println!("{}", hello);

    hello.push('!');
    println!("{}", hello);

    println!("Is emtpy {}", hello.is_empty());
    println!("Lenth: {}", hello.len());
    println!("Capacity: {}", hello.capacity());
    println!("Lowercase: {}", hello.to_lowercase());
    println!("Uppercase: {}", hello.to_uppercase());

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    let mut s = String::with_capacity(10);
    println!("Capacity: {}, Length: {}, Str: [{}]", s.capacity(), s.len(), s);

    s.push('a');
    s.push('b');
    println!("Capacity: {}, Length: {}, Str: [{}]", s.capacity(), s.len(), s);

    s.push('あ');
    println!("Capacity: {}, Length: {}, Str: [{}]", s.capacity(), s.len(), s);

    for c in s.chars() {
        println!("{}", c);
    }
}
