//this one is using structs 
#[derive(Debug)]
struct Rectangle {
        length: u32,
        breadth: u32,
    }
fn main(){
    let out = rect1(&Rectangle{length:20, breadth:30});
    
    println!("{:?}", out);
}
fn rect1(rectangle : &Rectangle) -> u32{
    rectangle.length * rectangle.breadth
}
/*we can also do this using normal variables and also by using tuple 
 * now using tuple
 * fn main(){
 * let Rectangle = (30,20);
 * let rect1 = area(Rectangle);
 * println!("{}",rect1);
 * }
 * fn area(dimenstions : (u32, u32))->u32{
 *      dimenstions.0 * dimenstions.1
 * }
 * */ 
