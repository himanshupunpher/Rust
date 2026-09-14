fn main() {
    let mut s1 = String::from("Hello");
    
    change(&mut s1)
}

fn change(some_str : &mut String){
    some_str.push_str(" Nigga");
    println!("{}",some_str)
}
