// mutability
// rust는 변수를 만들면 바꿀 수가 없음 바꾸고 싶으면 mut 사용
// 다음 코드 실행하면 오류남 

fn main() {
    let mut my_number = 10;
    my_number = 9;
    println!("{}", my_number)
}

// 이때 변수 바꾸고 싶으면 let 뒤에 mut 붙이면 됨



// shadowing : 같은 이름을 다시 쓰는 것
fn main() {
    let my_variable = 10;
    let my_variable = "My variable";
    println!("{}", my_variable)
}

// my_variable 변수가 두번 정의되면 앞에 있는게 가려짐 
// 즉, 계산은 되지만 위 경우는 계산 함수가 없기 때문에 없어지는거처럼 보임


// 여기에선 9가 실제로 double 함수에 의해 계산되고, 
// 프린트문에만 나오지 않으며 계산 결과는 저장되어 누적됨
fn double(input: i32) -> i32 {
    input * 2
}

fn triple(input: i32) -> i32 {
    input * 3
}

fn main() {
    let x = 9;
    let x = double(x);
    let x = triple(x);
    println!("{}", x);
}



//
fn main() {
    let my_variable = 9;
    println!("{}", my_variable);
    {
        let my_variable = "Some string";
        println!("{}", my_variable);
    }
    println!("{}", my_variable);
}