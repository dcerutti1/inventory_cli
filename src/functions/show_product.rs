use std::io;
use std::io::{stdout, Write};
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use crossterm::cursor::MoveTo;
use dialoguer::{Confirm, Input};
use dialoguer::console::Term;
use crate::functions::all_errors::{MyError};
use sqlx::{FromRow, SqlitePool};
use tabled::{Table, Tabled};
use tabled::settings::Style;

#[derive(Debug, FromRow, Tabled)]
struct Product {
    id: i32,
    name: String,
    quantity:u32
}
pub async fn show() -> Result<(), MyError> {
    let term = Term::stdout();
    let pool = SqlitePool::connect("./Database/prod.db").await?;

    let products: Vec<Product> = sqlx::query_as::<_, Product>("SELECT * FROM products")
    .fetch_all(&pool)
    .await?;
    
    if products.is_empty() {
        println!("==========No products found==========");

        let _ = Confirm::new()
            .with_prompt("Press Enter to continue...")
            .default(true)
            .interact()?;

        term.clear_screen();

    } else{
        
        let mut table = Table::new(&products); 
        
        let new = table.with(Style::modern());
         println!("{}", new);

        let _ = Confirm::new()
            .with_prompt("Press Enter to continue...")
            .default(true)
            .interact()?;
        term.clear_last_lines(100);
    }

    // Clear screen
    
    
    Ok(())
}
