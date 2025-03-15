use dialoguer::console::Term;
use dialoguer::Input;
use sqlx::SqlitePool;
use crate::functions::all_errors::{MyError};

pub(crate) async fn delete() -> Result<(), MyError> {
    let term = Term::stdout();
    
    let product_id:i32 = Input::new()
        .with_prompt("Enter product ID number")
        .interact_text()?;

     
    
    term.clear_last_lines(1);

    let pool = SqlitePool::connect("./Database/prod.db").await?;

    let result = sqlx::query(" DELETE FROM products WHERE id = ?")
        .bind(product_id)
        .execute(&pool)
        .await?;
    term.clear_last_lines(1);
    
    if result.rows_affected() > 0{
        println!(" ==========Product with ID {} deleted successfully!==========", product_id)
    } else {
        println!("==========no product found with ID {}==========", product_id)
    }
    term.clear_last_lines(1);
    Ok(())
}