// const = 값이 복사되어 들어감, 메모리 주소 없음 (쪽지를 복사해서 나눠줌)

// static = 프로그램이 실행되는 동안 메모리에 고정된 주소를 가지고 존재 (대자보 느낌)
// static 뒤에 mut 붙이면 지역 함수 안에서 unsafe{} 블록 만들어서 쓸 수 있긴함
// 근데 그럴일이 거의 존재 하지 않음 (메모리 주소가 겹치는 경우만 존재), 그리고 위험

// 둘다 변경 불가능한 값 선언, 둘다 전역 변수

// const 선언할때는 변수 이름이 대문자여야함 (소문자여도 가능은한데 귀찮음)
// 밑의 코드는 전역변수 예시 
const HIGH_SCORE: i32 = 20;  
static LOW_SCORE: i32 = 0;

fn print_high_score() {
    println!("The high score is {}", HIGH_SCORE);
}

fn main() {
    print_high_score();
}