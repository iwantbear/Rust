fn main() {
    // integer 에는 두가지 종류
    // Unsigned
    // signed Integer 
    // 뒤에 숫자는 비트 (bit)
    i8, i16, i32, i64, i128 and isize // -가 될 수 있는 수 
    u8, u16, u32, u64, u128 and usize // -가 될 수 없는 수 (예: 나이)
}


// 변수 선언할때는 let
// 하지만 밑 코드는 타입이 명시 되어 있지 않음
// 이때 Rust는 디폴트로 i32 
fn main() {
    let my_number = 100;
}


// 타입 명시할땐 : 붙여야함
fn main() {
    let my_number: u8 = 100;
}


// 다음과 같이 작성하면 두가지 문제가 발생 
// 1. third_number 변수 선언후 사용하지 않으므로 간단한 경고
// 2. my_other_number는 타입 선언하지 않았으므로 기본적으로 i32,
// 하지만 다른 타입 두가지를 더할 수 없음 
fn main() {
    let my_number: u8 = 100;
    let my_other_number = 50;
    let third_number = my_number + my_other_number;
}