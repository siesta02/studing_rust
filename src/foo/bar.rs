pub fn bar_func(){
    println!("Hello! Bar")
}
pub fn func1(){
    let mut name = String::from("ABC");
    println!("{}", name);
    name = func2(name);
    println!("{}", name);
}
pub fn func2(name: String) -> String{
    println!("{}", name);
    name
}