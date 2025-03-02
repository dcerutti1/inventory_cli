use std::error::Error;
use dialoguer::{
    Select,
    theme::ColorfulTheme
};
use functions::Add_product::add_product;
mod functions;

fn main() -> Result<(), Box<dyn Error>>{

    loop{
    let items = vec!["Add Product", "Delete Product", "Move Product", "Report","Update Product", "Settings", "Exit"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select one.")
        .items(&items)
        .interact()
        .unwrap();


        match selection {
            0 => add_product(),
            1 => Ok(println!("delete selected")),
            2 => Ok(println!("move selected")),
            3 => Ok(println!("report selected")),
            4 => Ok(println!("update selected")),
            5 => Ok(println!("settings selected")),
            6 => Ok(println!("exiting.. have a nice day.")),
            _ => Ok(println!("invalid selection"))
        }.expect("ERROR");

        if selection == 6{
            break;
        }
    }
    Ok(())
}
