fn main() {

    let mut head: Option<Box<LL>> = None;
    let mut tail: Option<Box<LL>> = None;




    create(&mut head, &mut tail, 1);

    create(&mut head, &mut tail, 2);

    create(&mut head, &mut tail, 3);

    create(&mut head, &mut tail, 4);

}


#[derive(Clone, Debug)]
struct LL {

    data: Box<i32>,
    next: Option<Box<LL>>
}



fn create(head : &mut Option<Box<LL>>,  tail : &mut Option<Box<LL>>, data: i32 )  {

    if head.is_none() {

        let new_node = Box::new(LL {
            data: Box::new(data),
            next: None,
        });
        *head = Some(new_node.clone()); 
        *tail = Some(new_node.clone());


    } else {

        let new_node = Box::new(LL {
            data: Box::new(data),
            next: None,
        });

        if let Some(tail_node) = tail.as_mut() {
            tail_node.next = Some(new_node.clone());
        }

        *tail = Some(new_node.clone());
    }
}


