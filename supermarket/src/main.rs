/* ##############################################
 *              NFT Chain Shop
 * A simple digital market with sellable object.
 * ##############################################
 */

mod user;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::*;
use std::str;
use user::*;

pub fn flush() {
    std::process::Command::new("clear").status().unwrap();
}

fn print_action() {
    flush();
    println!("
 _   _ ______ _______    _____          _    _ _____ _   _    _____ _    _  ____  _____  
| \\ | |  ____|__   __|  / ____|   /\\   | |  | |_   _| \\ | |  / ____| |  | |/ __ \\|  __ \\ 
|  \\| | |__     | |    | |       /  \\  | |__| | | | |  \\| | | (___ | |__| | |  | | |__) |
| . ` |  __|    | |    | |      / /\\ \\ |  __  | | | | . ` |  \\___ \\|  __  | |  | |  ___/ 
| |\\  | |       | |    | |____ / ____ \\| |  | |_| |_| |\\  |  ____) | |  | | |__| | |     
|_| \\_|_|       |_|     \\_____/_/    \\_\\_|  |_|_____|_| \\_| |_____/|_|  |_|\\____/|_|
            ");
}

fn manage_action() -> usize {

    return 0;
}

// The program takes as input the filename
// es: cargo run resources/catalog.json
fn main() {

    let args: Vec<String> = env::args().collect();
    let filename = &args[1]; // It must contains a filename

    let mut warehouse: HashMap<String,Sellable>;
    warehouse = load_json(filename.to_string());

    let mut budget: f64 = 2.50;

    let mut action_mesage: String = String::from("Welcome to our shop. Take a look around!\n");
    loop {

        print_action();
        println!("{}", action_mesage);
        println!("Your current budget: {:.2}€\n", budget);

        for object in warehouse.values() {
            println!("{}",object);
        }
 
        let mut buffer = String::new();

        print!("Tip the item that you want to buy: ");
        std::io::stdout().flush().unwrap();
        let stdin = std::io::stdin(); // We get `Stdin` here.
        let err = stdin.read_line(&mut buffer).err();
        
        if !err.is_none() {
            println!("{:?}", err);
            return;
        }
        buffer = buffer.replace("\n", "");
        
        let key = buffer.to_lowercase();
        if warehouse.contains_key(&key)  {
            
            let sellable = &warehouse[&key];

            if budget >= sellable.price {
                action_mesage = String::from("I love your money ＄＄＄\nWould you like to buy something else?");
                budget -= sellable.price;
                warehouse.remove(&key);
            }
            else {
                action_mesage = String::from("Oh! It seems that you have not enough money....");
            }
        }
        else {
            action_mesage = String::from("We don't sell that kind of things, man!");
        }
    }
}

pub fn load_json (filename: String) -> HashMap<String, Sellable> {

    let mut new_warehouse: HashMap<String,Sellable>  = HashMap::new();
    let contents = fs::read_to_string(filename)
                        .expect("Something went wrong reading the file");

    let catalog = json::parse(&contents.to_string()).unwrap();

    for i in 0..catalog["items"].len() {
        let name = catalog["items"][i]["name"].to_string();
        let price = catalog["items"][i]["price"].to_string();
        let description = catalog["items"][i]["description"].to_string();

        new_warehouse.insert(
            name.to_lowercase(),
            create_sellable(price.parse::<f64>().unwrap(), name, description)
        );
    }

    return new_warehouse;
}
