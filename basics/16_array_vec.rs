// Collection types
// Array, 배열 = [] 
// array는 인덱스 사용함
// array는 보통 수정하지 않을 값들만 사용할때 씀 (예로 month나 그런것들)
// 또한 array를 출력하고 싶으면 println! 문에서 "{}"가 아니라 "{:?}" 써야함


// 밑의 array, array2는 크기가 다르기 때문에 다른 타입임 즉, 크기가 같아야 비교가능
fn main() {
    let array = ["one", "two"];  // &str; 2
    let array2 = ["one", "two", "five"];  // &str; 3

    // 이건 진짜 개꿀팁인데 만약 array 변수의 타입을 알고 싶으면 
    // 밑의 코드처럼 변수명.아무이름(); 쓰면됨
    // 즉 .tasdfads() 함수는 존재하지 않음 아무렇게 쓰면 터미널에서 오류나면서 보임
    array.tasdfads();

    // array의 크기를 모른다고 가정할때 예를 들어 인덱스 3을 알고 싶으면
    // array.get() 함수 사용해서 알 수 있음 () 안에 인덱스 번호 넣으면 됨
    // 존재하면 Some, 존재하지 않으면 None
    println!("{:?}", array.get(3));
}


// array는 buffer 만들때 많이 쓰임
fn main(){
    let array = [0; 640];
    println!("{:?}", array);
}


// Slices
// 컴파일러는 스택의 크기를 알 수 없음 따라서 &를 붙어야 슬라이싱 할 수 있음
// [a..b] : 이렇게 할 시 b의 인덱스는 포함하지 않음
// [a..=b] :  b의 인덱스도 포함함
// [..] : 배열 전부 볼 수 있음 
// [a..] : a 부터 끝까지
// [..b] : 처음부터 b전까지

fn main() {
    let seasons = ["봄", "여름", "가을", "겨울"];

    println!("{:?}", &seasons[0..2]);
    println!("{:?}", &seasons[0..=2]);
    println!("{:?}", &seasons[..]);
    println!("{:?}", &seasons[2..]);
    println!("{:?}", &seasons[..2]);
}


// array (vec도 마찬가지로 동작)에 담긴 값을 가져오고 싶을때 다음과 같이 하면 됨
fn main() {
    let my_array = ["a", "b", "c"];
    let [a, b, c] = my_array;
    println!("Item is a : {}", a);
}

// 위 코드처럼 사용하면 오류는 안나지만 b, c를 사용하지 않기에 경고가 뜸
// 다음과 같이 하면 b, c를 무시할 수 있음
fn main() {
    let my_array = ["a", "b", "c"];
    let [a, _, _] = my_array;
    println!("Item is a : {}", a);
}
//------------------------------------------------------------------
// Vec(vector)
// array와 달리 수정가능한 값을 사용할때 사용함 즉, array와 차이점은 수정 가능 여부
// array와 마찬가지로 프린트문 에서 "{}"가 아닌 "{:?}" 사용해야함

// 밑의 코드는 String 타입의 변수 두개 만든 후, Vec에 하나씩 담음
// Vec은 메모리 공간이 처음엔 0 그리고 데이터 하나 담으면 4, 모자르면 2의 n으로 늘어남
fn main() {

    let name1 = "Windy".to_string();
    let name2 = "Comesy".to_string();

    // 이 문법은 빈 Vec 만들때 사용함
    let mut my_vec = Vec::new();

    println!("Space for my_vec: {:?}", my_vec.capacity()); // 0
    my_vec.push(name1);
    println!("Space for my_vec: {:?}", my_vec.capacity()); // 4
    my_vec.push(name2);

    println!("My cats are{:?}", my_vec);
}


// Vec::new 보다 편리한건 vec![], 실제로는 이걸 더 많이 사용함

fn main() {
    let name1 = "Windy".to_string();
    let name2 = "Comesy".to_string();

    let my_vec = vec![name1, name2];
    println!("My cats are{:?}", my_vec);
}