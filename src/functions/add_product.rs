use std::error::Error;
use std::time::Duration;
use dialoguer::console::Term;
//this function adds product to database after validation check.
use dialoguer::{Confirm, Input};
use indicatif::ProgressBar;
use rand::Rng;
use crate::functions::validation_check::product_check;
use sqlx::{
    Row,
    sqlite::{ SqlitePool},
};
use crate::functions::all_errors::{MyError};
use tokio;

pub async fn add_product() -> Result<(), MyError> {
    let term = Term::stdout();
    loop {

        let product_name: String = Input::new()
            .with_prompt("Enter product name")
            .interact_text()?;

        if product_name.to_lowercase() == "exit" {
            println!("Exiting...");
            term.clear_last_lines(100);
            break return Ok(());
        }
        
        let product_location: String = Input::new()
            .with_prompt("Enter product location")
            .interact_text()?;

        if product_location.to_lowercase() == "exit" {
            println!("Exiting...");
            term.clear_last_lines(100);
            break return Ok(());
        }
        term.clear_last_lines(1);

        let product_quantity: u32 = Input::new()
            .with_prompt("Enter product quantity")
            .interact_text()?;
        term.clear_last_lines(1);


//catch panic and loops to try again



           let result = product_check(&product_name, product_quantity).is_ok();

                if result{
                    // Generate a random ID for crate.
                    let mut rng = rand::thread_rng();
                    let crate_id:i32 = rng.gen_range(1..=1000);

                    let pool =connect_db().await;


                    let pool_result = match pool {
                        Ok(pool) => pool,
                        Err(e) => {
                            println!("Error creating pool: {}", e);
                            return Err(MyError::Database(sqlx::Error::RowNotFound));
                        }
                    };

                        if let Err(e) = add_to_db(&pool_result, crate_id, product_name, product_quantity, product_location).await{
                        println!("{}", e);
                    }

                   break return Ok(());
                }else {
                    println!("please enter a valid product name or quantity");
                }




    }

pub async fn connect_db() -> Result<SqlitePool, Box<dyn Error>> {
    
    let pool = SqlitePool::connect("./Database/prod.db").await?;
    Ok(pool)
}
    
async fn add_to_db(pool: &SqlitePool, id: i32, name: String, quantity: u32, location: String) -> Result<(), MyError> {
        let term = Term::stdout();


    let username_result = sqlx::query("SELECT u.username FROM users u JOIN sessions s ON u.id = s.user_id LIMIT 1")
        .fetch_optional(pool)
        .await;

    let username = match username_result {
        Ok(Some(row)) => row.try_get::<String, _>("username")?,
        Ok(None) => "Unknown".to_string(),
        Err(_) => {
            eprintln!("Error fetching username from sessions table");
            "Unknown".to_string() // Default value if an error occurs
        }
    };

    

   let input = sqlx::query(" INSERT INTO products (id, name, quantity, location, added_by) VALUES (?,?,?,?,?)")
        .bind(id)
        .bind(name)
        .bind(quantity)
        .bind(location)
       .bind(username)
        .execute(pool)
        .await?;



        println!("==========Added successfully==========");
    
        
        let _ = Confirm::new()
            .with_prompt("Press Enter to continue...")
            .default(true)
            .interact()?;

        term.clear_last_lines(100);
        
        
        Ok(())
}

   

   

}
