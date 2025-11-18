// 이게 기본 주석임 
fn main() {
    println!("Hello, world!");
}


/// documents - 컴파일시, html로 뜸 (설명서 같이 보여주는것)
// documents 주석(///) 사용시
// 1. cargo doc 명령어로 html 문서로 변환, 저장
// 2. cargo doc --open 명령어로 확인 가능
struct Book;


// /* */ 이 주석은 특정 부분만 주석처리하는 것 
fn main() {
    let x/*: i16*/ = 10;
}

// 변수 이름이 underbar (_)로 시작하면 컴파일러가 해석 불가능
fn main() {
    let _x/*: i16 */ = 10;
}



