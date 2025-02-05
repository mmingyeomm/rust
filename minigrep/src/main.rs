use std::env;
use std::fs;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Arguments::parse_args(&args).unwrap_or_else(|err| {
        println!("Problem passing Arguments : {}" ,err);
        process::exit(1);

    });

    let contents: String = fs::read_to_string(config.file).expect("something wrong with file");

    println!("{}", &contents);

}

struct Arguments {
    query: String,
    file: String
}   


impl Arguments{

    fn parse_args(args: &[String])-> Result<Arguments, &str>{

        if (args.len() < 3){
            return Err("not enough arguments")
         }

        let query: String = args[1].clone();
        let file: String = args[2].clone();

        Ok(Arguments {query, file})

    }

}