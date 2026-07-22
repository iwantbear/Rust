// pointers
// 책에서 각 챕터가 몇페이지에 있는지 알려주는 목차 같은 느낌으로 생각하면 됨
// 예를 들어 다음과 같은 느낌 

// Chapter     Page
// Chapter 1    1
// Chapter 2    15
// Chpater 3    23

// 즉 포인터는 실제 값을 저장하는게 아니라 값이 있는 메모리 주소를 가리키는 것

fn main() {
    let my_number = 15; // i32
    let single_reference = &my_number; // &i32
    let double_reference = &single_reference; // &&i32
    let five_reference = &&&&&my_number; // &&&&&i32
}

// 비유하자면 다음과 같음 
// my_number = 실제 책 내용 (15 페이지)
// single_reference = "15 페이지를 봐라" 라고 가리키는 목차
// double_reference = "목차의 목차"를 가리키는 느낌