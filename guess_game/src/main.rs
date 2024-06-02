use std::io;
use rand::Rng;

fn main() {
    let guess_list = ["grapes", "banana", "orange"];
    let mut rng = rand::thread_rng();

    let index = rng.gen_range(0..guess_list.len());
    let random_fruit = guess_list[index];


    let mut input = String::new();
    println!("guess the fruit among grapes, banana, orange");
    loop{
        match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let fruit_selected = input.trim().to_lowercase();
                    println!("fruit-selected: {}", fruit_selected);

                    if !guess_list.contains(&&fruit_selected.as_str()){
                        println!("fruit entered does not found");
                        continue;
                    }
                    if guess_checker(&fruit_selected,random_fruit){
                        println!("The random fruit is: {}", random_fruit);
                        println!("you are winner");
                        break;
                    }else{
                        println!("The random fruit is: {}", random_fruit);
                        println!("retry")
                    }
                }
                Err(error) => {  
                    println!("Error: {}", error);
                }
            }
        }   
}

fn guess_checker(fruit_selected:&str,random_fruit:&str)->bool{
    return fruit_selected==random_fruit;
}