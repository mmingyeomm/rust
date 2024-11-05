pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

struct rectangle {
    width:  u32, 
    height: u32 
}

impl rectangle {
    fn can_hold(&self, other: &rectangle) -> bool {
        self.width >= other.width && self.height >= other.height


    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

    }

    #[test]
    fn fits(){
        let rectangle1= rectangle{width: 5, height: 3}; 
        let rectangle2= rectangle{width: 5, height: 3}; 
        
        let result = rectangle1.can_hold(&rectangle2);
        assert_eq!(result , true)

    }
}
