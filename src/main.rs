fn main() {
    if_test(18);
    if_test(5);
    if_else_test(20);
    if_else_test(2);
}

fn if_test(x:i32){
    if x > 10 {
        println!("the passed argument is greater than 10, And its value is {}",x);
    }
}

fn if_else_test(x:i32) {
    if x > 10 {
        println!("the passed argument is greater than 10, And its value is {}",x);
    } else {
        println!("The passed argument is less than the integer 10, the value of the argument is {}",x);
    }
}