use std::error::Error;
use dialoguer::{
    Select,
    theme::ColorfulTheme
};
mod functions;
use functions::add_product::add_product;
use functions::delete_product::delete;
use functions::show_product::show;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{

    loop{
    let items = vec!["Add Product", "Delete Product", "View Product", "Report","Update Product", "Settings", "Exit"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select one.")
        .items(&items)
        .interact()
        .unwrap();


        match selection {
            0 => add_product().await,
            1 => delete().await,
            2 => show().await,
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
//todo!("in add_product fn, add the final step which is writing generating an ID  and saving to database");