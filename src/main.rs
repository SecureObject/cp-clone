use std::fs;

fn main() {
    let file = "./file.txt";

    match fs::canonicalize(file){
        Ok(f) => println!("The file path! {}", f.display()),
        Err(_) => { println!("File not found"); 
            println!("Do You want me to create the file ? ");

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            create_file(&input);
            
        },
    }

    println!("Do you want to copy the file ?");
    
    let mut input2 = String::new();

    std::io::stdin().read_line(&mut input2).expect("Failed to read usr input");

//    println!("Input :{}", input);

    match input2.as_str() {
        "y" | "Y" | "yes" | "Yes" => {
        println!("Please enter the copied file name ?");
        let mut name = String::new();

        std::io::stdin().read_line(&mut name).unwrap();

        fs::copy("file.txt", &name);
    },
    _=> {
        println!("See u again! Bye");
        },
      }
}


fn create_file(input: &String){

            match input.as_str(){
                "y" | "Y"| "yes" | "Yes" => {
            std::fs::OpenOptions::new()
                .read(true).write(true)
                .create(true)
                .open("file.txt");
            println!("File created!");
                },
                _=> {
                println!("Sorry! I can't help you!");
                },
            }

}
