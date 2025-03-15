use std::error::Error;
use std::time::Duration;
use dialoguer::console::Term;
//this function adds product to database after validation check.
use dialoguer::Input;
use indicatif::ProgressBar;
use rand::Rng;
use crate::functions::validation_check::product_check;
use sqlx::{
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
            break
            return Ok(());
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

                        if let Err(e) = add_to_db(&pool_result, crate_id, product_name, product_quantity).await{
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
    //generate id here
async fn add_to_db(pool: &SqlitePool, id: i32, name: String, quantity: u32,) -> Result<(), Box<dyn Error>> {
    sqlx::query(" INSERT INTO products (id, name, quantity) VALUES (?,?,?)")
        .bind(id)
        .bind(name)
        .bind(quantity)
        .execute(pool)
        .await?;

        println!("==========Added successfully==========");
        Ok(())
}



}
