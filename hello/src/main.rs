
fn main() {
    // // scalars
    // let age:u32 = 18; // unsigned 32 bit int
    // let num:i32= -1; // default for numbers if type isnt specified
    // let temperature :f64= 36.6; // 64 bit floating point number
    // let n = 3;

    // // println!("{} {} {} {}", age, num, temperature, n);
    // // compound type

    // // tuple
    // let person :(&str, u32) = ("Philo", 34);

    // // array
    // let scores; [i32; 3] = [90, 5, 1];
    greet("Philo");
    println!("{}",add(1,4));

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter
        }
    };
    println!("res: {}",result);

    struct User {
        name: String,
        age: u32,
    }

    let user = User {
        name: String::from("philo"),
        age : 34
    };
    impl User {
        fn describe(&self) -> String{
            format!("{}", self.name)
        }
    }
    println!("user name {}", user.describe())

}

fn greet(name: &str){
    println!("Hello {} ", name)
}

fn add(a: i32, b:i32)->i32{
    return a + b;
}

