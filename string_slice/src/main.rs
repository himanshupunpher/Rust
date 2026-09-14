fn main() {
    let s = String::from("Hello World");

    let slice = first_word(&s);

    println!("{}",slice);
}
fn first_word(s : &String) -> &str{
    let bytes = s.as_bytes();

    for (i,&items) in bytes.iter().enumerate(){
        if items == b' '{
            return &s[..i];
        }
    }
    &s[..]
}
