
struct Point{
    x: i32,
    y: i32,
}
struct Rect{
    width: u32,
    height: u32,
}
union MyUnion{
    f1: u32,
    f2: u32,
}
enum Color{
    Red,
    Green,
    Blue,
}
macro_rules! log{
    ($x:expr) => { println!("{}", $x); }
}

fn main() {
    println!("Hello, world!");
    let n = 0;
    const MAX_POINTS: u32 = 100_000;
    println!("{}", n);
    println!("{}", MAX_POINTS);
    let p = Point {x:100, y:200};
    println!("{} {}", p.x, p.y);
    let u = MyUnion{f1: 123};
    unsafe {
        println!("{}", u.f1);
        println!("{}", u.f2);
    }
    //let color = Color::Red;
    let tup = (10, "20", 30);
    println!("{} {} {}", tup.0, tup.1, tup.2);

    let arr = [10, 20, 30];
    println!("{} {} {}", arr[0], arr[1], arr[2]);
    for v in &arr{
        println!("here {}", v);
    }
    let mut vect = vec![10, 20, 30];
    vect.push(40);
    println!("{} {} {} {}", vect[0], vect[1], vect[2], vect[3]);
    for v in &vect{
        println!("{}", v);
    }
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("x", 10);
    map.insert("y", 20);
    map.insert("z", 30);
    println!("{} {} {}", map["x"], map["y"], map["z"]);
    for (k, v) in &map{
        println!("{} {}", k, v);
    }
    let mut name: &str = "Yamada";
    name = "Tanaka";
    let mut name = String::from("Yamada");
    name = "Tanaka".to_string();
    name.push_str(" Taro");
    println!("{}", name);

    let p: Box<Point> = Box::new(Point{x: 100, y: 200});
    println!("{} {}", p.x, p.y);
    impl Drop for Point{
        fn drop(&mut self){
            println!("Bye!");
        }
    }
    let s = String::from("ABCDEFGH");
    let s1 = &s[0..3];
    let s2 = &s[3..6];
    println!("{} {}", s1, s2);

    let a = [10, 20, 30, 40, 50, 60, 70, 80];
    let a1 = &a[0..3];
    let a2 = &a[3..6];
    println!("{:?} {:?}", a1, a2);

    fn add(x: i32, y: i32) -> i32{
        return x + y;
    }
    println!("add() is here - {}", add(10, 20));
    let square = | x: i32 | {
        x * x
    }; //let square なので;は必須
    println!("{}", square(9));

    let msg = String::from("Hello");
    let func = move || {
        println!("{}", msg);
    };
    func();
    log!("ABC");
    if n==1 {
        println!("One");
    }else if n == 2 {
        println!("Two");
    }else {
        println!("Other");
    }
    let s = if n == 1 { "Ok!"} else { "NG!"};
    println!("{} {}", s, n);

    let mut n = 0;
    while n < 10 {
        n += 1;
    }
    for i in 0..10 {
        println!("{}", i);
    }
    let mut n = 0;
    loop{
        n += 1;
        if n == 2 {
            continue;
        }
        if n == 8 {
            break;
        }
    }
    let x = 2;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("more"),
    }
    impl Rect{
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    let r = Rect { width: 200, height: 300};
    println!("{}", r.area());

}
