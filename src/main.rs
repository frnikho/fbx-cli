use crate::app::App;
use crate::handlers::auth::Auth;
use crate::handlers::config::Config;
use crate::handlers::device::Device;
use crate::handlers::settings::Settings;
use crate::handlers::system::System;
use crate::handlers::vm::Vm;
use crate::models::args::{
    AuthCommands, Commands, ConfigCommands, DevicesCommands, SettingsCommands, SystemCommands,
    VmSubCommands,
};

mod app;
mod client;
mod config;
mod handlers;
mod models;
mod services;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = App::default();
    app.initialize().await;
    let _ = match app.cli.cmd.clone() {
        Commands::Vm { cmd } => match cmd {
            VmSubCommands::List(args) => Vm::list(&mut app, &args).await,
            VmSubCommands::Get(args) => Vm::get(&mut app, &args).await,
        },
        Commands::Device { cmd } => match cmd {
            DevicesCommands::List => Device::list(&mut app).await,
        },
        Commands::Auth { cmd } => match cmd {
            AuthCommands::Status => Auth::status(&mut app).await,
            AuthCommands::Logout => Auth::logout(&mut app).await,
            AuthCommands::Login(args) => Auth::login(&mut app, args).await,
            AuthCommands::SetUrl(args) => Auth::set_url(&mut app, args).await,
            AuthCommands::Init(_args) => Auth::init(&mut app).await,
        },
        Commands::Settings { cmd } => match cmd {
            SettingsCommands::Reset { .. } => Settings::reset(&mut app).await,
            SettingsCommands::SetUrl { .. } => Settings::set_url(&mut app).await,
            SettingsCommands::Discover { .. } => Settings::discover(&mut app).await,
            SettingsCommands::GetUrl => Settings::get_url(&mut app).await,
        },
        Commands::System { cmd } => match cmd {
            SystemCommands::Reboot => System::reboot(&mut app).await,
            SystemCommands::Shutdown => System::shutdown(&mut app).await,
            SystemCommands::Get => System::get(&mut app).await,
            SystemCommands::Update => System::get_update_status(&mut app).await,
            SystemCommands::Language {..} => todo!()
        },
        Commands::Config { cmd } => match cmd {
            ConfigCommands::SetUrl(args) => Config::change_url(&mut app, args.url).await,
            ConfigCommands::Info => Config::show(&mut app).await,
            ConfigCommands::Reset(args) => Config::reset(&mut app, args.confirm).await,
        },
        Commands::Info => {
            println!("{:?}", app);
            println!(
                "{}",
                format_args!("{} - {}", whoami::username(), whoami::distro())
            );
            Ok(())
        },
        _ => Ok(())
    }
    .map_err(|x| {
        println!("{:?}", x);
    });
    app.save();
    Ok(())
}
