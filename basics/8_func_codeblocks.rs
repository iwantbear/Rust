// 변수에 숫자 넣을때 타입 꼭 써야함
// 세미콜론 없음 : 그거 반환
// 세미콜론 있음 : 해당 줄 실행하고 끝

// 기본 1 : 함수가 값을 반환 → 밖에서 사용 가능
fn give_number(one: i32, two: i32) -> i32 {
    one * two
}

fn main() {
    let my_number = give_number(9, 8);
    println!("{}", my_number);
}




// 기본 2 : 함수가 값을 반환하지 않고 내부에서 바로 출력
fn print_number(one: i16, two: i16) {
    let multiplied = one * two;
    println!("{}", multiplied);
}

fn main() {
    print_number(7, 8);
}




// 기본 3  
fn print_number(one: i16, two: i16) -> i16 {
    let multiplied_by_ten = {
        let first_number = 10;     // 이건 함수 안에서만 10으로 사용가능
        first_number * one * two
    };
    multiplied_by_ten
}

fn main() {
    let my_number = print_number (7, 8);
    println!("{}", my_number);
}