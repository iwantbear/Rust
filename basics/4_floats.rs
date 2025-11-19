// float의 경우 변수 선언할때 뒤에다 타입을 넣음
// 언더바를 넣어도 되며 갯수는 무제한 (에러 아님)
fn main() {
    let my_number = 9u8;
    let my_number2 =9_u8;
    let my_number3 = 9_________________u8; 
    
    // 예를 들어 단위를 표시하고 싶을때 언더바 사용하면 좋음
    let my_number4 = 1_000_000_000u64;
}



// 숫자에 . 들어가면 무조건 float
// f32 , f64 두가지 
// float + int 하려면 타입을 as 로 맞춰준다
// float을 as 사용해서 integer로 바꾸면 소숫점 밑은 다 버림 따라서 해당 코드 출력은 18
// integer를 as 사용해서 float으로 바꾸면 소숫점을 버리지 않음 
fn main() {
    let my_number = 9.6; // f64
    let my_number2 = 9; // i32
    println!("{}", my_number as i32 + my_number2)
}