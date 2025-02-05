use std::thread;
use std::time::Duration;



fn simulated_expensive_calculation(intensity: u32) -> u32{  
    println!("calculating slowly.."); 
    thread::sleep(Duration::from_secs(2));
    intensity

}



fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;


    fn generate_workout(simulated_intensity: u32, simulated_random_number: u32){

        if (simulated_intensity < 25){
            println!("")
        }

    }



}
