use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use crate::functions;
use functions::all_errors::{MyError};
use crate::functions::users::{Database, UserActions};

pub async fn admin_menu() -> Result<(), MyError> {
    
    loop {
        let items = vec!["add user", "list users", "delete user", "edit user permissions", "exit"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Please select one.")
            .items(&items)
            .interact()?;

        match selection {
            0 => {
                Database::new().await?.create_user().await?;
                let _ = Ok::<(), MyError>(());


            },
            1 => {
                Database::new().await?.list_users().await?;
                let _ = Ok::<(), MyError>(());

            },
            2 => {
                Database::new().await?.delete_user().await?;
                let _ = Ok::<(), MyError>(());

            },
            3 => {
                println!("Please select one.");
                let _ = Ok::<(), MyError>(());

            },
            
            4 => {
                println!("Please select one.");
                let _ = Ok::<(), MyError>(());

            },
            _ => {
                println!("Please select one.");
                let _ = Ok::<(), MyError>(());

            },

        }
        
        if selection == 4{
            break;
        }
       
    }
    Ok(())    
    }