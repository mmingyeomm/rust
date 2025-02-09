fn main() {

   
   let authorization_status :Option<&str> = None; 
   let is_admin : bool = false; 
   let group_id: Result<u8, _> =  "34".parse(); 

   let x : Option<u32> = None;
   let y : u32 = 5; 



    match x {

        Some(50) => println!("match 50"), 
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("else is ")

    }; 


        





   if let Some(status) = authorization_status {
        println!("is some")
   } else if is_admin {
        println!("authorization = admin")
   } else if let Ok(group_id) = group_id{
        if group_id > 30 {
            println!(" over 30 "); 
        } else {
            println!(" under 30 ")
        }

   }
 





}
