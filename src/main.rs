use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|i| i=="ascii"){
    println!("test");
    } else {
        println!("racc")
    } 
    
    
}
