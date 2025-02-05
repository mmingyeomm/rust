
#[derive(Debug)]
struct Rectangle {
    width: u8,
    height: u8 
}

impl Rectangle {

    fn check_fit(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }

}



#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn larger_can_hold_smaller(){

        let large = Rectangle{
            width: 5,
            height: 4,
        };

        let small = Rectangle{
            width: 1,
            height: 2,
        };

        assert_eq!(true, large.check_fit(&small));


    }

}
