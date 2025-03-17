use dialoguer::console::Term;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use crate::{auth, functions};
use functions::all_errors::{MyError};
use crate::functions::users::{Database, UserActions};
use auth::sessions::session_check;
pub async fn admin_menu() -> Result<(), MyError> {

    let term = Term::stdout();
    term.clear_last_lines(100);
    
    session_check().await?;
    
    loop {
        let items = vec!["add user", "list users", "delete user", "edit user permissions", "exit"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Please select one.")
            .items(&items)
            .clear(true)
            .interact()?;
        
        term.clear_last_lines(1);
        
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
                println!("exiting..");
                let _ = Ok::<(), MyError>(());

            },
            _ => {
                println!("Please select one.");
                let _ = Ok::<(), MyError>(());

            },

        }
        
        if selection == 4{
            term.clear_last_lines(100);
            break;
        }
       
    }
    Ok(())    
    }