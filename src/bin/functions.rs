fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);
}

fn change(some_stirng: &mut String) {
    
    some_stirng.push_str(", world");
}
