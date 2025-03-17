use dialoguer::console::Term;
use dialoguer::{Confirm, Input};
use sqlx::SqlitePool;
use crate::functions::all_errors::{MyError};

pub(crate) async fn delete() -> Result<(), MyError> {
    let term = Term::stdout();
    
        let product_id: i32 = Input::new()
            .with_prompt("Enter product ID number")
            .interact_text()?;

        
        let pool = SqlitePool::connect("./Database/prod.db").await?;

        let result = sqlx::query(" DELETE FROM products WHERE id = ?")
            .bind(product_id)
            .execute(&pool)
            .await?;


        if result.rows_affected() > 0 {
            println!(" ==========Product with ID {} deleted successfully!==========", product_id)
        } else {
            println!("==========no product found with ID {}==========", product_id)
        }
        let _ = Confirm::new()
            .with_prompt("Press Enter to continue...")
            .default(true)
            .interact()?;

        term.clear_last_lines(100);
        
    Ok(())
    
}