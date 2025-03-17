use dialoguer::{Confirm, Input};
use dialoguer::console::Term;
use sqlx::{query_as, FromRow, SqlitePool};
use tabled::{Table, Tabled};
use tabled::settings::Style;
use crate::functions::all_errors::MyError;

#[derive(Debug, FromRow, Tabled)]
struct Product {
name: String,
location: String
}
pub async fn pick_list() -> Result<(), MyError> {
    let term = Term::stdout();
    
    let pool = SqlitePool::connect("./Database/prod.db").await?;

    loop {
        let search_term: String = Input::new()
            .with_prompt("Enter product name for the pick list.")
            .interact_text()?;
        
        if &search_term == "exit"{
            break;
        }

        let products: Vec<Product> = query_as::<_, Product>("SELECT name, location FROM products WHERE name LIKE ?")
            .bind(search_term)
            .fetch_all(&pool).await?;
        
      

        if products.is_empty() {
            println!("==========No products found==========");

            let _ = Confirm::new()
                .with_prompt("Press Enter to continue...")
                .default(true)
                .interact()?;

            term.clear_last_lines(100);
        } else {
            let mut table = Table::new(&products);

            let new = table.with(Style::modern());
            println!("{}", new);

            let _ = Confirm::new()
                .with_prompt("Press Enter to continue...")
                .default(true)
                .interact()?;

            let term = Term::stdout();
            term.clear_last_lines(100);

        }

    }
    
    
    Ok(())
}