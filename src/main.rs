#![allow(dead_code)]

use crate::app::App;
use crate::handlers::auth::Auth;
use crate::handlers::device::Device;
use crate::handlers::lcd::Lcd;
use crate::handlers::system::System;
use crate::handlers::vm::Vm;
use crate::models::args::{AuthCommands, Commands, DevicesCommands, LcdCmds, LcdSetCmds, SystemCommands, VmSubCommands};
use crate::models::exception::ClientError;
use crate::terminal::{CliDisplayArg};

mod app;
mod client;
mod config;
mod handlers;
mod models;
mod services;
mod terminal;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = App::default();
    app.initialize().await;
    let a = match app.cli.cmd.clone() {
        Commands::Lcd { cmd} => match cmd {
            LcdCmds::Get => Lcd::get(&mut app).await,
            LcdCmds::Set { cmd } => match cmd {
                LcdSetCmds::Brightness { brightness } => Lcd::set_brightness(&mut app, brightness).await,
                LcdSetCmds::Orientation { orientation } => Lcd::set_orientation(&mut app, orientation).await,
                LcdSetCmds::HideWifiKey { hide } => Lcd::hide_password(&mut app, hide).await,
            },
        },
        Commands::Vm { cmd } => match cmd {
            VmSubCommands::List(args) => Vm::list(&mut app, &args).await,
            VmSubCommands::Get(args) => Vm::get(&mut app, &args).await,
        },
        Commands::Device { cmd } => match cmd {
            DevicesCommands::List => Device::list(&mut app).await,
            DevicesCommands::Get(args) => Device::get_device(&mut app, &args).await,
            DevicesCommands::Update(args) => Device::update(&mut app, args).await,
        },
        Commands::Auth { cmd } => match cmd {
            AuthCommands::Status => Auth::status(&mut app).await,
            AuthCommands::Logout => Auth::logout(&mut app).await,
            AuthCommands::Login(args) => Auth::login(&mut app, args).await,
            AuthCommands::SetUrl(args) => Auth::set_url(&mut app, args).await,
        },
        Commands::System { cmd } => match cmd {
            SystemCommands::Reboot => System::reboot(&mut app).await,
            SystemCommands::Shutdown => System::shutdown(&mut app).await,
            SystemCommands::Get => System::get(&mut app).await,
            SystemCommands::Update => System::get_update_status(&mut app).await,
            SystemCommands::Language {..} => todo!(),
        },
        Commands::Info => {
            println!("{:?}", app);
            println!(
                "{}",
                format_args!("{} - {}", whoami::username(), whoami::distro())
            );
            Err(ClientError::UnknownError("Not implemented"))
        },
        _ => Err(ClientError::UnknownError("Not implemented"))
    }
    .map_err(|x| {
        println!("{:?}", x);
    }).unwrap();
    println!("{}", a.json());
    println!("{}", a.stdout(CliDisplayArg { no_color: false}));
    app.save();
    Ok(())
}
