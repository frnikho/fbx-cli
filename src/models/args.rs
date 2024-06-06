use clap::{ArgAction, Args, Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(name = "fbx")]
#[command(help_template = "\
{bin} {version}
{author}
{about}

\x1b[1mUSAGE:\x1b[0m
    {usage}

\x1b[1mCOMMANDS:\x1b[0m
{subcommands}

\x1b[1mOPTIONS:\x1b[0m
{options}

Pour plus d'informations, utilisez '{bin} <commande> --help'")]
#[command(about = "Une CLI pour gérer les commandes fbx", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: Commands,

    #[arg(short, long, help = "Affiche des informations supplémentaires", default_value = None, action = ArgAction::SetTrue)]
    pub verbose: Option<bool>,

    #[arg(short, long, help = "Supprime les messages d'erreur", default_value = None, action = ArgAction::SetTrue)]
    pub quiet: Option<bool>,

    #[arg(short, long, help = "Desactiver la couleur", default_value = None, action = ArgAction::SetTrue)]
    pub no_color: Option<bool>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Vm {
        #[clap(subcommand)]
        cmd: VmSubCommands,
    },
    Device {
        #[clap(subcommand)]
        cmd: DevicesCommands,
    },
    Config {
        #[clap(subcommand)]
        cmd: ConfigCommands,
    },
    Auth {
        #[clap(subcommand)]
        cmd: AuthCommands,
    },
    Settings {
        #[clap(subcommand)]
        cmd: SettingsCommands,
    },
    System {
        #[clap(subcommand)]
        cmd: SystemCommands,
    },
    Info,
    Dhcp {
        #[clap(subcommand)]
        cmd: DhcpCommands,
    },
    Ftp {
        #[clap(subcommand)]
        cmd: FtpCommands,
    },
    Port {
        #[clap(subcommand)]
        cmd: PortCommands,
    },
    Lcd,
    Notification,
    Wifi {
        #[clap(subcommand)]
        cmd: WifiCommands
    },
    Vpn {
        #[clap(subcommand)]
        cmd: VpnCommands
    },
}

/*
  VM
*/

#[derive(Subcommand, Debug, Clone)]
pub enum VmSubCommands {
    List(VmListArgs),
    Get(VmGetArgs),
}

#[derive(Args, Debug, Clone)]
pub struct VmListArgs {}

#[derive(Args, Debug, Clone)]
pub struct VmGetArgs {
    uuid: String,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ConfigCommands {
    Info,
    SetUrl(ConfigSetUrlArgs),
    Reset(ConfigResetArgs),
}

#[derive(Args, Debug, Clone)]
pub struct ConfigSetUrlArgs {
    pub url: String,
}

#[derive(Args, Debug, Clone)]
pub struct ConfigResetArgs {
    #[arg(short, long, help = "Réinitialise les paramètres de l'application", default_value = None, action = ArgAction::SetTrue)]
    pub confirm: bool,
}

#[derive(Args, Clone)]
pub struct CreateVmArgs {
    name: String,
    description: String,
}

#[derive(Args, Clone)]
pub struct DeleteVmArgs {
    pub uuid: String,
}

#[derive(Args, Debug, Clone)]
pub struct InfoVmArgs {
    pub uuid: String,
}

#[derive(Args, Debug, Clone)]
pub struct InitArgs {
    #[arg(short, long, help = "Définit l'adresse de la Freebox", default_value = None)]
    pub url: Option<String>,
    #[arg(short, long, help = "Découvre les périphériques Freebox", default_value_t = true, action = ArgAction::SetFalse)]
    pub discover: bool,

    #[arg(
        short,
        long,
        help = "Délai d'attente pour la découverte des périphériques",
        default_value_t = 5
    )]
    pub time: u64,

    #[arg(short, long, help = "Ignore les erreurs de découverte", default_value = None, action = ArgAction::SetTrue)]
    pub without_confirm: bool,

    #[arg(short = 's', long = "bypass-status", help = "Bypass status checking for app", default_value = None, action = ArgAction::SetTrue)]
    pub bypass_status: bool,

    #[arg(
        long = "app_id",
        help = "Override default app_id",
        default_value = "com.nikho.fbx-cli"
    )]
    pub app_id: String,

    #[arg(long = "device", help = "Override default device_name")]
    pub device: Option<String>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum DevicesCommands {
    List,
}

#[derive(Subcommand, Debug, Clone)]
pub enum DhcpCommands {
    Get,
}

#[derive(Subcommand, Debug, Clone)]
pub enum WifiCommands {
    Get,
}

#[derive(Subcommand, Debug, Clone)]
pub enum FtpCommands {
    Get,
}

#[derive(Subcommand, Debug, Clone)]
pub enum VpnCommands {
    Get,
}

#[derive(Subcommand, Debug, Clone)]
pub enum PortCommands {
    Get,
}

#[derive(Subcommand, Debug, Clone)]
pub enum AuthCommands {
    Status,
    Logout,
    Login(AuthLoginArgs),
    SetUrl(AuthSetUrlArgs),
    Init(InitArgs),
}

#[derive(Args, Debug, Clone)]
pub struct AuthSetUrlArgs {
    pub url: String,

    #[arg(long, help = "Skip verification of url", default_value_t = false, action = ArgAction::SetTrue)]
    pub skip_verification: bool,
}

#[derive(Args, Debug, Clone)]
pub struct AuthLoginArgs {
    pub url: String,

    #[arg(long, help = "override app_id", default_value = None, action = ArgAction::SetTrue)]
    pub app_id: Option<String>,

    #[arg(long, help = "override app_version", default_value = None, action = ArgAction::SetTrue)]
    pub app_version: Option<String>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum SettingsCommands {
    Discover {
        #[arg(short, long, help = "Découvre les périphériques Freebox", default_value = None, action = ArgAction::SetTrue)]
        all: Option<bool>,
    },
    SetUrl {
        url: String,
        #[arg(short, long, help = "Test or not the provided url", default_value = None, action = ArgAction::SetTrue)]
        bypass_test: Option<bool>,
    },
    GetUrl,
    Reset {
        #[arg(short, long, help = "Réinitialise les paramètres de l'application", default_value = None, action = ArgAction::SetTrue)]
        confirm: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum SystemCommands {
    #[clap(about = "Shutdown the freebox")]
    Shutdown,
    #[clap(about = "Reboot the freebox")]
    Reboot,
    #[clap(alias = "info", about = "Get system information")]
    Get,
    #[clap(about = "Display update state")]
    Update,
    #[clap(about = "Change/display system language")]
    Language {

    }
}
