

fn main() {

    let head = LinkedList::new(1); 

    let x = LinkedList::insert_head(2, head); 



    let mut y = LinkedList::insert_head(3, x); 


    y.delete(3);

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
            next: Some(Box::new(head_before)) 
        }
    }

    fn fetch(&self) -> i32 {
        self.value
    } 

    fn delete(&mut self, value : i32)  { 

        

        let mut current = self; 
        loop { 

            match &current.next {
                Some(x) => current = x.next.unwrap() ,
                None => return, 

            }


        }

        loop {
            match current.next.as_mut() {  // No & here! as_mut() on Option
                Some(next_box) => {
                    // next_box: &mut Box<LinkedList>
                    // Box<T> auto-derefs to &mut T
                    current = next_box;  // This works! Types match
                }
                None => return,
            }
        }

    }


    fn fetch_tail(&self) -> i32 {
        let mut current = self;
        loop {
            match current.next.as_ref() {
                None => return current.value,
                Some(next) => current = next,
            }
        }
    }

    

} 

// 양방향 링크드 리스트 ADT: new/ tail찾기 / insert_head/ delete  