use std::error::Error;
use std::iter::Product;
use sqlx::{FromRow, SqlitePool};
use tabled::{Table, Tabled};
use tabled::settings::Style;

#[derive(Debug, FromRow, Tabled)]
struct product {
    id: i32,
    name: String,
    quantity:u32
}
pub async fn show() -> Result<(), Box<dyn Error>> {
    let pool = SqlitePool::connect("./Database/prod.db").await?;

    let products: Vec<product> = sqlx::query_as::<_, product>("SELECT * FROM products")
    .fetch_all(&pool)
    .await?;
    if products.is_empty() {
        println!("==========No products found==========");
    } else{
        let mut table = Table::new(&products); // Create the table
        let new = table.with(Style::modern());
         println!("{}", new)
    }

    Ok(())
}
