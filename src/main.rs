use std::boxed::Box;
use std::thread;
use std::time::Duration;
use foo::bar as bbaarr;
struct Point{
    x: i32,
    y: i32,
}
struct Rect{
    width: u32,
    height: u32,
}
struct Rect2<T>{
    width: T,
    height: T,
}
struct Dog{}
struct Cat{}
struct Counter {
    max: u32,
    count: u32,
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

mod foo;

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

    trait Printable {fn print(&self);}
    impl Printable for Rect {
        fn print(&self){
            println!("width:{}, height{}", self.width, self.height)
        }
    }
    let r = Rect {width: 200, height: 300 };
    r.print();

    impl<T> Printable for Rect2<T> where T: std::fmt::Display {
        fn print(self: &Rect2<T>){
            println!("{}x{}", self.width, self.height)
        }
    }
    let r1: Rect2<i32> = Rect2{width: 100, height: 200};
    let r2: Rect2<i64> = Rect2{width: 100, height: 200};
    r1.print();
    r2.print();

    trait Animal{fn cry(&self);}
    impl Animal for Dog {fn cry(&self){println!("Bow-wow!");}}
    impl Animal for Cat {fn cry(&self){println!("Miaow");}}
    fn get_animal(animal_type: &str) -> Box<dyn Animal> {
        if animal_type == "dog" {
            return Box::new(Dog {});
        }else {
            return Box::new(Cat {});
        }
    }

    get_animal("dog").cry();
    get_animal("cat").cry();

    impl Counter {
        fn new(max: u32) -> Counter{
            Counter {max: max, count:0}
        }
    }
    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item>{
            self.count += 1;
            if self.count < self.max {
                Some(self.count)
            }else{
                None
            }
        }
    }
    let counter = Counter::new(10);
    for c in counter{
        println!("{}", c);
    }

    let th = thread::spawn(|| {
        for _i in 1..10 {
            println!("A");
            thread::sleep(Duration::from_millis(100));
        }
    });
    th.join().unwrap();
    println!("Finished");

    let str = String::from("ABC");
    let th = thread::spawn(move || {
        for _i in 1..10 {
            println!("{}", str);
            thread::sleep(Duration::from_millis(100));
        }
    });
    th.join().unwrap();
    println!("Finishe");

    // await async　
    //crate 

    bbaarr::bar_func();

    let mut a = 123;
    let p = &mut a;
    *p = 456;
    println!("{}", a);

    bbaarr::func1();

    
}
