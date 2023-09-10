use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|i| i=="ascii"){
   println!(r"/\ /\");
   println!(r"''-   _");
   println!(r"() ()\  ,'_\");
   println!(r"( . ) )/._./");
   println!(r"_)-(_).--'");                                            
    } else {
        println!("racc");
    } 
    
    
}
