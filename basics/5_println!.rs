// 기본적인 프린트문 
fn main() { 
    println!("Hello, world!");
}



// 변수 넣을때는 뒤에 순서대로 
fn main() { 
    let my_name = "David";
    let my_age = 42;
    println!("My name is {} and my age is {}", my_name, my_age);
}

// 혹은 {} 안에 변수 넣기 (String Interpolation 이라고함)
fn main() { 
    let my_name = "David";
    let my_age = 42;
    println!("My name is {my_name} and my age is {my_age}");
}

// 변수 대신에 인덱스로 넣을 수 있음 
fn main() {
    let city = "Seoul";
    let population = 9_987_987;
    println!("I love {0}, this {0} had a population of {1}!", city, population);
}



// 42 앞에 return 써도되고 안써도됨
fn give_age() -> i32 {
    return 42
}

fn main() { 
    let my_name = "David";
    println!("My name is {} and my age is {}", my_name, give_age());
}