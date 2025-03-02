use std::error::Error;
use std::time::Duration;
//this function adds product to database after validation check.
use dialoguer::Input;
use indicatif::ProgressBar;
use crate::functions::validation_check::product_check;
use std::panic;

pub fn add_product() -> Result<(), Box<dyn Error>> {
    loop {

        let product_name: String = Input::new()
            .with_prompt("Enter product name")
            .interact_text()
            .map_err(|_| "❌ Failed to read product name")?;


        let product_quantity: u32 = Input::new()
            .with_prompt("Enter product quantity")
            .interact_text()
            .map_err(|_| "❌ Invalid quantity. Please enter a number")?;



//catch panic and loops to try again



           let result = product_check(&product_name, product_quantity).is_ok();

                if result{
                    println!("product '{}' added successfully.", product_name);
                   break return Ok(());
                }else {
                    println!("please enter a valid product name or quantity");
                }





    }






}
