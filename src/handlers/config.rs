use crate::app::App;
use crate::models::exception::ClientError;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use inquire::Confirm;
use url::Url;

pub struct Config;

impl Config {
    pub async fn change_url(app: &mut App, url: String) -> Result<(), ClientError> {
        match Url::parse(&url) {
            Ok(url) => {
                println!("Url: {:?}", url);
                app.config.pref.base_url = url.to_string();
                println!("Url updated !");
            }
            Err(_) => {
                println!("Invalid url provided !");
            }
        }
        Ok(())
    }

    pub async fn reset(app: &mut App, confirm: bool) -> Result<(), ClientError> {
        if !confirm {
            Confirm::new("Êtes-vous sûr de vouloir réinitialiser la configuration ?")
                .prompt()
                .unwrap();
        }
        app.config.reset().unwrap();
        println!("Configuration réinitialisée !");
        Ok(())
    }

    pub async fn show(_app: &mut App) -> Result<(), ClientError> {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(180)
            .set_header(vec![
                Cell::new("Header1").add_attribute(Attribute::Bold),
                Cell::new("Header2").add_attribute(Attribute::Bold).fg(Color::Green),
                Cell::new("Header3"),
            ])
            .add_row(vec![
                Cell::new("This is a bold text").add_attribute(Attribute::Bold),
                Cell::new("This is a green text").fg(Color::Green),
                Cell::new("This one has black background").bg(Color::Black),
            ])
            .add_row(vec![
                Cell::new("Blinky boi").add_attribute(Attribute::SlowBlink),
                Cell::new("This table's content is dynamically arranged. The table is exactly 80 characters wide.\nHere comes a reallylongwordthatshoulddynamicallywrap"),
                Cell::new("COMBINE ALL THE THINGS")
                    .fg(Color::Green)
                    .bg(Color::Black)
                    .add_attributes(vec![
                        Attribute::Bold,
                        Attribute::SlowBlink,
                    ])
            ]);

        println!("{table}");
        Ok(())
    }
}
