fn main() {
    let s= call_backfunction(11,22) ;
    println!("The sum is {}",s);

}
//random commit
fn call_backfunction(x:i32,y:i32) -> i32 {

    println!("The value of x is {}",x);
    println!("The value of y is {}",y);
    return x+y;

}
