use std::error::Error;
use dialoguer::{Confirm, Input};
use dialoguer::console::Term;
use crate::functions::all_errors::{MyError};
use sqlx::{FromRow, SqlitePool};
use tabled::{Table, Tabled};
use tabled::settings::Style;

#[derive(Debug, FromRow, Tabled)]
struct product {
    id: i32,
    name: String,
    quantity:u32
}
pub async fn show() -> Result<(), MyError> {
    let term = Term::stdout();
    let pool = SqlitePool::connect("./Database/prod.db").await?;

    let products: Vec<product> = sqlx::query_as::<_, product>("SELECT * FROM products")
    .fetch_all(&pool)
    .await?;
    
    if products.is_empty() {
        println!("==========No products found==========");

        let _ = Confirm::new()
            .with_prompt("Press Enter to continue...")
            .default(true)
            .interact()?; // Wait for Enter // Waits for Enter

        term.clear_last_lines(2); 
         
    } else{
        
        let mut table = Table::new(&products); // Create the table
        let new = table.with(Style::modern());
         println!("{}", new);

        let _ = Confirm::new()
            .with_prompt("Press Enter to continue...")
            .default(true) 
            .interact()?;

        let term = Term::stdout();
        term.clear_last_lines(1);
            
    }
    
    Ok(())
}
