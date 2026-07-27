// ------------------------------------------------------------------------
// mutability
// rust는 변수를 만들면 바꿀 수가 없음 바꾸고 싶으면 mut 사용
// 이때 변수 바꾸고 싶으면 let 뒤에 mut 붙이면 됨

fn main() {
    let mut my_number = 9;
    my_number = 10;
    println!("{}", my_number)
}


// ownership (소유권 기본 개념)
// 하나의 변수는 하나의 데이터만 갖고 있을 수 있음

// & = immutable reference / shared reference (수정 불가능)
// &mut = mutable reference / unique reference (수정 가능)
// 이 경우에는 딱 한 변수에만 적용 가능하며, &, &mut는 동시에 사용 불가능

// 1. 가변 변수 my_number = 9 저장
// 2. &mut로 my_number를 가변 참조로 만들어서 num_ref에 저장 
// 이떄 num_ref는 my_number의 메모리 주소를 가리킴
// 3. *num_ref로 역참조 하여 my_number 값을 10에서 9로 변경함
fn main() {
    let mut my_number = 9;
    let num_ref = &mut my_number;

    *num_ref = 10;
    println!("Number is now {my_number}}")
}

// 첫줄의 number 변수는 mut이 있기 때문에 number = 20; 같이 본인은 수정가능
// 둘째줄의 number_ref는 number를 빌려오기만함 (수정 불가능)
// 세번째줄의 number_change는 number 변수를 빌려오고 수정가능하지만 바로 위에 
// &가 있기 때문에 오류가 남 만약 둘째줄이 없으면 괜찮

// 밑의 코드는 10이라는 데이터를 number 변수가 소유하고 있었는데 
// number_ref가 빌려오려고 &number를 사용함 근데 한 데이터는 하나의 변수만 사용가능
// 그니까 바로 뒤에서 number_change 변수가 &mut를 사용하면 안됨
fn main() {
    let mut number = 10;
    let number_ref = &number;
    // number_change는 참조, 변경할 수 있는 자격이 됨
    let number_change = &mut number;  
    *number_change += 10;   // *은 역참조, 값을 변경하려면 이거까지 써야함
    println!("{number_ref}");
}

// 이 코드는 10이라는 데이터를 number가 소유하고 있음
// 그리고 number_change 변수가 &mut를 써서 빌려오고 수정한뒤 바로 버려버림
// 그리고 number_ref라는 새 변수가 다시 빌려오기 때문에 가능
fn main() {
    let mut number = 10;
    let number_change = &mut number;  
    *number_change += 10;   
    let number_ref = &number;
    println!("{number_ref}");
}
// ------------------------------------------------------------------------
// shadowing : 같은 이름을 다시 쓰는 것

// my_variable 변수가 두번 정의되면 앞에 있는게 가려짐 
// 즉, 계산은 되지만 위 경우는 계산 함수가 없기 때문에 없어지는거처럼 보임
fn main() {
    let my_variable = 10;
    let my_variable = "My variable";
    println!("{}", my_variable)
}


// 결과 = 대한민국, 8
// country_ref는 country를 빌리기만 했으니까 그대로 "대한민국"
// country = 8로 shadowing 하면 이전건 덮는 느낌이고 새로 8로 변함
fn main() {
    let country = "대한민국";
    let country_ref = &country;
    let country = 8;
    println!("{country_ref},{country}");
}


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


