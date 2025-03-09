use std::error::Error;
use dialoguer::{Select, theme::ColorfulTheme, Input};
use sqlx::SqlitePool;

mod functions;
mod auth;

use functions::add_product::add_product;
use functions::delete_product::delete;
use functions::show_product::show;
use functions::banner::show_banner;
use crate::auth::user_creation::{Create, User};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let pool = SqlitePool::connect("./Database/prod.db").await?;
    show_banner();
    
    loop{
    let items = vec!["Add Product", "Delete Product", "View Product", "Report","Update Product","Add user", "Settings", "Exit"];

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
            5 => User::create(&pool).await.map(|user|{
                println!("created user");
            }),
            6 => Ok(println!("settings selected")),
            7 => Ok(println!("exiting.. have a nice day.")),
            _ => Ok(println!("invalid selection"))
        }.expect("ERROR");

        if selection == 6{
            break;
        }
    }
    Ok(())
}
