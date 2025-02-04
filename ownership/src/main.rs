fn main() {

    let a: String = String::from("new");



    takes_ownership(a);


    println!("Hello, world! {}" , a);
}


fn takes_ownership(some_string: String){

    println!("{}", some_string);

}