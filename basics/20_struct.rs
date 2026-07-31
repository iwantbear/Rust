// struct(구조체) = 서로 다른 데이터 타입을 묶어서 하나의 새 타입을 만드는 것

// unit struct = 아무 데이터를 저장하지 않는 struct
// 바이트 수 = 0

// 예를 들어 아래 두 변수는 타입이 다르면서 각각 i32, string임
// let alpha = 8;
// let beta = "Hi";

// struct FileDirectory; 라고 해버리면 FileDirectory 라는 새 타입을 만든것
// 즉 역할을 구별하기 위한것임 (디비에서 같은 타입을 삭제하지 못하게 하기 위함) 

// 예시, 선언은 다음과 같이 하면 됨
struct FileDirectory;

fn takes_file_directory(input: FileDirectory) {
    println!("I got a file directory");
}

fn main() {
    let x = FileDirectory;
    takes_file_directory(x);
}


// 바이트 보고 싶을떈 "std::mem::size_of_val" 함수 사용하면 됨
// 변수 앞에 & 붙여야함
struct FileDirectory;

fn main() {
    let x = FileDirectory;
    println!("The size is {}", std::mem::size_of_val(&x));
}








// tuple struct = 튜플에 이름을 붙여 변수처럼 만드는것 
// 컴파일러 입장에선 타입만 같으면 변수 이름 가지곤 차이점(의미)를 알 수 없음
// struct는 새로운 타입이기 때문에 컴파일러가 뭔지 모름 그래서 출력하고 싶으면
// #[derive(Debut)]를 구조체 앞에 써야함, 동시에 프린트 문에는 {:?} 로도 써야함 (안되면 {:#?})

struct FileDirectory;
#[derive(Debug)]
struct Colour(u8, u8, u8);

fn main() {
    let my_colour = Colour(20, 50, 100);
    println!("The second colour is {:?}", my_colour);
}









// named struct = 
// 일반 구조체를 밑의 코드처럼 세개 (인구, 수도, 대통령이름) 정의했으면
// 함수 불러올때 세가지 다 정의해야함
struct Country {
    population: u32, 
    capital: String, 
    leader_name: String
}

fn main() {
    let canada = Country {
        population: 35_000_000,
        capital: "Ottawa".to_string(),
        leader_name: "Justin Trudeau".to_string()
    };
    println!("The population is: {} \nThe capital is: {}", canada.population, canada.capital);
}


// 위 코드에서 Country 구조체 전부를 보고 싶으면(프린트문으로 출력하려면) 다음과 같음
#[derive(Debug)]
struct Country {
    population: u32, 
    capital: String, 
    leader_name: String
}

fn main() {
    let canada = Country {
        population: 35_000_000,
        capital: "Ottawa".to_string(),
        leader_name: "Justin Trudeau".to_string()
    };
    println!("The country is: {:?}", canada);
}








// alignment 개념 
// Rust는 어떤 구조체에 담긴 변수의 위치가 "4의 배수"인 곳에 정렬되는 것을 선호
// 예를 들어서
// 칸이 8칸있는 책장이 있을때 u8 = 바이트 이기 때문에 0번째 칸에 옴
// u32 = 4바이트인데 1,2,3,4 번째 칸에 오는걸 싫어해서 4,5,6,7 번째 칸에 오게됨 
// 즉 1,2,3 칸은 비어있고 이걸 패딩하기 때문에 이 구조체의 전체 크기는 8바이트가 됨

// 꿀팁들
// std::mem::size_of_val; 함수를 프린트문에 쓰지 않고 use 써서 위로 빼버려도 알 수 있음
use std::mem::size_of_val;

struct Numbers {
    one: u8,
    two: u8,
    three: u8,
    four: u32
}

#[derive(Debug)]
struct Country {
    population: u32, 
    capital: String, 
    leader_name: String
}

fn main() {
    let population = 35_000_000;
    let capital = "Ottawa".to_string();
    let leader_name = "Justin Trudeau".to_string();

    let my_country = Country {
        // 만약 밑의 줄을 popultation: population, 으로 써버린다면
        // 굳이 안써도 되기에 그냥 population, 으로만 써도 됨
        population,
        capital,
        leader_name
    };
    println!("Country is {} bytes in size", size_of_val(&my_country));

    // 여기에서 0,1,2번엔 각각 8,19,20 데이터가 채워짐
    // 위에서 말했듯이 구조체에선 데이터가 4의 배수 자리에 오는걸 선호하기 때문에
    // [one, two, three, P, four, four, four, four]
    // 실제 값이 저장될땐 [8, 19, 20, P, 30, 0, 0, 0] 
    // 이건 엔디언 개념으로 16진수로 표현해서 넣는 규칙
    let numbers = Numbers {
        one: 8,     // 1바이트
        two: 19,    // 1바이트
        three: 20,  // 1바이트
        four: 30    // 4바이트
    };

    println!("The size is: {}", size_of_val(&numbers));
}
