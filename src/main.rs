
use dialoguer::{Select, theme::ColorfulTheme};
use dialoguer::console::Term;
use functions::all_errors::{MyError};
mod functions;
mod auth;
use functions::{
    add_product::add_product,
    delete_product::delete,
    show_product::show,
    banner::show_banner,
    admin_menu::admin_menu
    
};

use crate::auth::login::{Authenticate, User};


#[tokio::main]
async fn main() -> Result<(), MyError>{
    let term = Term::stdout();
    show_banner();
    User::login().await?;
    
    
    


    loop{
         
    let items = vec!["Add Product", "Delete Product", "View Product", "Admin settings","Update Product","Settings", "Exit"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select one.")
        .items(&items)
        .clear(true)
        .interact()?;
        term.clear_last_lines(1);

        match selection {
            0 => add_product().await,
            1 => delete().await,
            2 => show().await,
            3 => admin_menu().await,
            4 => Ok(println!("update selected")),
            5 => Ok(println!("settings selected")),
            6 => Ok(println!("exiting.. have a nice day.")),
            _ => Ok(println!("invalid selection"))
        }.expect("ERROR");

        if selection == 6{
            break;
        }
        let term = Term::stdout();
        term.clear_screen();
    }
    Ok(())
}
