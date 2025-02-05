fn main() {


}




#[derive(PartialEq, Debug)]
struct Shoe
{
    size : u32, 

    style: String,
}

struct Counter {
    count: u32
}

impl Counter{
    fn new() -> Counter {
        Counter {count : 0}
    }
}


impl Iterator for Counter {
    type Item = u32; 

    fn next(&mut self) -> Option<Self::Item> {

        if self.count < 5 {
            self.count += 1; 
            Some(self.count)
        } else {
            None
        }
    }

}


fn shoes_in_my_size(mysize: u32, shoes: Vec<Shoe> ) ->  Vec<Shoe> {
    
    shoes.into_iter().filter(|x| x.size == mysize).collect()


}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(10, shoes);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}