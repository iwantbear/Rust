// uninitialized variable
// control flow

// 다음 코드는 오류가 남. 러스트는 초기화가 되지 않은 즉, 아무것도 없는 값 사용 불가능
fn main() {
    let my_number: u8;
    println!("{my_number}");
}




//  이런 코드는 가능하긴한데 쓸데가 많진 않음
fn main() {
    let my_number: u8;
{
    my_number = 9;
}

    println!("{my_number}");
}



// 루프 안에서도 초기화 안한 변수를 사용할 수 있다는걸 보여주는 예시
// main 함수에서 my_number 값을 명시 안함
// 43 이라는 값을 loop 함수로 보내고 결과를 x에 저장함
// 루프 함수에서 43을 받아서 1씩 늘어나다가 50으로 딱 떨어지면 끝남
// 다시 main 함수로 돌아와서 50을 x에 저장하고 my_number로 넘긴 후 출력

// 참고로 %는 어떤수로 나눴을때 그 나머지를 구하는 것
// 이 방식은 Rust 답지 않음 
// Rust 같으려면 break; 부분을 break counter;로 바꾸면 됨
fn loop_then_return(mut counter: i32) -> i32 {
    loop {
        counter += 1;
        if counter % 50 == 0 { 
            break;
        }
    }
    counter
}

fn main() {
    let my_number;
    {
        let x = loop_then_return(43);
        my_number = x
    }
    println!("{my_number}");
}