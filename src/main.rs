#![allow(dead_code)]

use anyhow::anyhow;
use clap::Parser;
use crate::app::App;
use crate::handlers::auth::Auth;
use crate::handlers::device::Device;
use crate::handlers::dhcp::Dhcp;
use crate::handlers::lcd::Lcd;
use crate::handlers::system::System;
use crate::handlers::vm::Vm;
use crate::handlers::wifi::Wifi;
use crate::models::args::{AuthCommands, Cli, Commands, DevicesCommands, DhcpCmds, DhcpDnsCmds, DhcpLeaseCmds, FtpCmds, LanguageCmds, LcdCmds, LcdSetCmds, SystemCommands, VmSubCommands, WifiCommands, WifiConfigCmds, WifiGuestCmds, WifiMacFilterCmds, WifiWpsCmds};
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
async fn main() -> Result<(), anyhow::Error> {
    let mut app = App::default();
    let cli = Cli::parse();
    app.initialize().await;
    let a = match cli.cmd {
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
            DevicesCommands::Set(args) => Device::update(&mut app, args).await,
            DevicesCommands::WakeUp(args) => Device::wak_on_lan(&mut app, args).await,
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
            SystemCommands::Language {cmd} => match cmd {
                LanguageCmds::Get => System::get_language(&mut app).await,
                LanguageCmds::Set(args) => System::update_language(&mut app, &args.lang).await,
            },
            SystemCommands::Ftp {cmd} => match cmd {
                FtpCmds::Get(args) => System::get_ftp_config(&mut app, args).await,
                FtpCmds::Set(args) => System::update_ftp_config(&mut app, args).await,
            },
        },
        Commands::Dhcp {cmd} => match cmd {
            DhcpCmds::Get(args) => Dhcp::get_config(&mut app, args).await,
            DhcpCmds::Set(args) => Dhcp::update_config(&mut app, args).await,
            DhcpCmds::Lease{cmd} => match cmd {
                DhcpLeaseCmds::List(args) => Dhcp::list_lease(&mut app, args).await,
                DhcpLeaseCmds::Get(args) => Dhcp::get_lease(&mut app, args).await,
                DhcpLeaseCmds::Create(args) => Dhcp::add_lease(&mut app, args).await,
                DhcpLeaseCmds::Update(args) => Dhcp::update_lease(&mut app, args).await,
                DhcpLeaseCmds::Delete(args) => Dhcp::delete_lease(&mut app, args).await,
            },
            DhcpCmds::Dns {cmd} => match cmd {
                DhcpDnsCmds::List(args) => Dhcp::list_dns(&mut app, args).await,
                DhcpDnsCmds::Set(args) => Dhcp::set_dns(&mut app, args).await,
                DhcpDnsCmds::Add(args) => Dhcp::add_dns(&mut app, args).await,
                DhcpDnsCmds::Remove(args) => Dhcp::remove_dns(&mut app, args).await,
                DhcpDnsCmds::Clear(args) => Dhcp::clean_dns(&mut app, args).await,
            }
        }
        Commands::Wifi {cmd} => match cmd {
            WifiCommands::Guest {cmd} => match cmd {
                WifiGuestCmds::List => todo!(),
                WifiGuestCmds::Get(_args) => todo!(),
                WifiGuestCmds::Create(_args) => todo!(),
                WifiGuestCmds::Delete(_args) => todo!(),
            },
            WifiCommands::Wps {cmd} => match cmd {
                WifiWpsCmds::List => todo!(),
                WifiWpsCmds::Start(_args) => todo!(),
                WifiWpsCmds::Stop(_args) => todo!(),
                WifiWpsCmds::Clear => todo!(),
            }
            WifiCommands::Diagnostic(_args) => todo!(),
            WifiCommands::MacFilter {cmd} => match cmd {
                WifiMacFilterCmds::List => todo!(),
                WifiMacFilterCmds::Get(_args) => todo!(),
                WifiMacFilterCmds::Create(_args) => todo!(),
                WifiMacFilterCmds::Delete(_args) => todo!(),
                WifiMacFilterCmds::Update(_args) => todo!(),
            }
            WifiCommands::QrCode(args) => Wifi::get_qr_code(&mut app, args).await,
            WifiCommands::Planning(_args) => todo!(),
            WifiCommands::Get(args) => Wifi::list_wifi(&mut app, args).await,
            WifiCommands::Scan(_args) => todo!(),
            WifiCommands::Config {cmd} => match cmd {
                WifiConfigCmds::Reset(_args) => todo!(),
                WifiConfigCmds::Set => todo!(),
            }
        }
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
    let result = a.stdout(CliDisplayArg::default());
    println!("Result: \n{}\n", result);

    let raw_result = a.raw(CliDisplayArg::default());
    println!("Raw Result: \n{}\n", raw_result);

    let json_result = a.json();
    println!("Json Result: \n{}", json_result);
    match result.code {
        terminal::CliResultCode::Success => Ok(()),
        terminal::CliResultCode::Error => Err(anyhow!("abc")),
    }
}
