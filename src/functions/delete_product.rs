use std::error::Error;
use dialoguer::Input;
use sqlx::SqlitePool;

pub(crate) async fn delete() -> Result<(), Box<dyn Error>> {

    let product_id: i32 = Input::new()
        .with_prompt("Enter product ID number")
        .interact_text()
        .map_err(|_| "❌ Failed to read product ID")?;


    let pool = SqlitePool::connect("./Database/prod.db").await?;

    let result = sqlx::query(" DELETE FROM products WHERE id = ?")
        .bind(product_id)
        .execute(&pool)
        .await?;

    if result.rows_affected() > 0{
        println!(" ==========Product with ID {} deleted successfully!==========", product_id)
    } else {
        println!("==========no product found with ID {}==========", product_id)
    }

    Ok(())
}