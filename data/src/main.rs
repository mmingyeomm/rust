

fn main() {

    let head = LinkedList::new(1); 

    let x = LinkedList::insert_head(2, head); 

    let y = LinkedList::insert_head(3, x); 

    println!("{}", y.fetch());

}


struct LinkedList{
    value : i32, 
    next : Option<Box<LinkedList>>

}



impl LinkedList{

    fn new (value :i32) -> LinkedList {
        LinkedList {
            value: value,
            next: None
        }
    }

    fn insert_head(value : i32 , head_before : LinkedList) -> LinkedList {

        LinkedList {
            value: value,
            next: Some( Box::new(head_before)) 
        }
    }

    fn fetch(&self) -> &i32 {
        &self.value
    } 

    fn fetch_tail(&self) -> i32 {

        let mut current = self; 

        loop {
            match current.next {
                None => return current.value, 
                Some(next) => current = next 
            }
        }
    }

} 

// 양방향 링크드 리스트 ADT: new/ tail찾기 / insert_head/ delete  