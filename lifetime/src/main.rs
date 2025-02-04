use::std::fmt::Display; 


fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str , ann: T)-> &'a str where T: Display,{


    println!("announcement : {}", ann);

    if (x.len() > y.len()) {

        return x; 
    } else {

        return y;
    }

}





fn main() {

    


}


