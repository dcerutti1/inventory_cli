//this function checks product validation to avoid typos.

use std::fs;
use serde::{Deserialize, Serialize};

use std::io::{self, ErrorKind};

#[derive(Serialize, Deserialize, Debug)]
struct Products {
    name: String,
    quantity: u32,
}

pub fn product_check(name: &str, quantity: u32) -> Result<(), io::Error> {
    let file_path = "./Config/Products.json";
    loop {

        // Read file contents safely
        let data = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(err) if err.kind() == ErrorKind::NotFound => {
                return Err(io::Error::new(ErrorKind::NotFound, "Products.json file not found"));
            }
            Err(err) => return Err(err),
        };

        // Parse JSON safely
        let products: Vec<Products> = match serde_json::from_str(&data) {
            Ok(parsed) => parsed,
            Err(_) => return Err(io::Error::new(ErrorKind::InvalidData, "Failed to parse Products.json")),
        };

        // Normalize case before comparison
        let target_name = name.to_lowercase();

        let found = products.iter().any(|product| {
            product.name.to_lowercase() == target_name && product.quantity == quantity
        });

        if !found {
            return Err(io::Error::new(ErrorKind::NotFound, format!("Product '{}' not found or quantity mismatch", name)));
        } else
        { break;

        }
    }
    Ok(())
}
