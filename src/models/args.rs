use clap::{ArgAction, Args, Parser, Subcommand, ValueEnum};
use crate::models::freebox::configuration::lan::LanHostType;

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
        cmd: DhcpCmds,
    },
    Ftp {
        #[clap(subcommand)]
        cmd: FtpCommands,
    },
    Port {
        #[clap(subcommand)]
        cmd: PortCommands,
    },
    Lcd {
        #[clap(subcommand)]
        cmd: LcdCmds,
    },
    Notification,
    Wifi {
        #[clap(subcommand)]
        cmd: WifiCommands
    },
    Vpn {
        #[clap(subcommand)]
        cmd: VpnCommands
    },
    Switch {
        #[clap(subcommand)]
        cmd: SwitchCommands
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

/***
 * LCD
 ***/

#[derive(Subcommand, Debug, Clone)]
pub enum LcdCmds {
    #[clap(alias = "info", about = "Retrieve and display LCD Info")]
    Get,
    #[clap(about = "Change LCD parameters")]
    Set {
        #[clap(subcommand)]
        cmd: LcdSetCmds,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum LcdOrientation {
    #[clap(name = "normal", aliases = ["0", "base"])]
    Normal,
    #[clap(name = "inverse", aliases = ["180"])]
    Inverse,
    #[clap(name = "right", aliases = ["90"])]
    FlipRight,
    #[clap(name = "left", aliases = ["270"])]
    FlipLeft,
}

impl LcdOrientation {
    pub fn value(&self) -> u16 {
        match self {
            LcdOrientation::Normal => 0,
            LcdOrientation::Inverse => 180,
            LcdOrientation::FlipRight => 90,
            LcdOrientation::FlipLeft => 270,
        }
    }

}

#[derive(Subcommand, Debug, Clone)]
pub enum LcdSetCmds {
    #[clap(about = "Set the brightness (0-100)", disable_help_flag = true)]
    Brightness {
        #[arg(value_name = "0-100")]
        brightness: u8,
    },
    #[clap(about = "Set the orientation (normal/inverse/left/right)", disable_help_flag = true)]
    Orientation {
        #[arg(value_name = "(normal/inverse/left/right)")]
        orientation: LcdOrientation,
    },
    #[clap(about = "Hide or not the wifi key (true/false)", disable_help_flag = true)]
    HideWifiKey {
        #[arg(value_name = "TRUE/FALSE")]
        hide: String,
    },
}

/***
 * Config
 ***/

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
/**
    Init
**/

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

/**
    Device
**/

#[derive(Subcommand, Debug, Clone)]
pub enum DevicesCommands {
    List,
    Get(DeviceGetArgs),
    Set(DeviceUpdateArgs),
    WakeUp(WakeUpArgs)
}

#[derive(Args, Debug, Clone)]
pub struct WakeUpArgs {
    #[arg(alias = "id")]
    pub name: String,
    pub password: Option<String>
}

#[derive(Args, Debug, Clone)]
pub struct DeviceGetArgs {
    pub id: String,
}

#[derive(Args, Debug, Clone)]
pub struct DeviceUpdateArgs {
    #[arg(alias = "id")]
    pub name: String,
    #[arg(short, long = "name", help = "Mettre à jour le nom du périphérique")]
    pub new_name: Option<String>,
    #[arg(long = "type", help = "Mettre à jour le type du périphérique")]
    pub r#type: Option<LanHostType>,
    #[arg(long = "persistent", help = "Effacer l'appareil s'il n'est pas présent lors du prochain redémarrage de la freebox")]
    pub persistent: Option<bool>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum SwitchCommands {

}

/**
    Wifi
**/

#[derive(Subcommand, Debug, Clone)]
pub enum WifiCommands {
    Get(WifiGetArgs),
    Scan(WifiScanArgs),
    QrCode(WifiQrCodeArgs),
    Planning(WifiPlanningArgs),
    Config {
        #[clap(subcommand)]
        cmd: WifiConfigCmds,
    },
    MacFilter {
        #[clap(subcommand)]
        cmd: WifiMacFilterCmds
    },
    Diagnostic(WifiDiagArgs),
    Wps {
        #[clap(subcommand)]
        cmd: WifiWpsCmds,
    },
    Guest {
        #[clap(subcommand)]
        cmd: WifiGuestCmds
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum WifiMacFilterCmds {
    List,
    Get(MacFilterArgs),
    Update(MacFilterUpdateArgs),
    Delete(MacFilterArgs),
    Create(MacFilterCreateArgs)
}

#[derive(Args, Debug, Clone)]
pub struct MacFilterArgs {

}

#[derive(Args, Debug, Clone)]
pub struct MacFilterCreateArgs {

}

#[derive(Args, Debug, Clone)]
pub struct MacFilterUpdateArgs {

}

#[derive(Args, Debug, Clone)]
pub struct WifiGetArgs {}

#[derive(Args, Debug, Clone)]
pub struct WifiScanArgs {}

#[derive(Args, Debug, Clone)]
pub struct WifiQrCodeArgs {}

#[derive(Args, Debug, Clone)]
pub struct WifiPlanningArgs {}

#[derive(Subcommand, Debug, Clone)]
pub enum WifiConfigCmds {
    Set,
    Reset(WifiConfigResetArgs),
}

#[derive(Args, Debug, Clone)]
pub struct WifiConfigResetArgs {
    #[arg(long, help = "Reset all wifi configuration", default_value_t = false, action = ArgAction::SetTrue)]
    pub all: bool,
    pub ap: bool,
    pub bss: bool,
}

#[derive(Args, Debug, Clone)]
pub struct WifiFilterArgs {}

#[derive(Args, Debug, Clone)]
pub struct WifiDiagArgs {}

#[derive(Subcommand, Debug, Clone)]
pub enum WifiWpsCmds {
    Start(WifiStartArgs),
    Stop(WifiStopArgs),
    List,
    Clear,
}

#[derive(Args, Debug, Clone)]
pub struct WifiStartArgs {

}

#[derive(Args, Debug, Clone)]
pub struct WifiStopArgs {

}

#[derive(Subcommand, Debug, Clone)]
pub enum WifiGuestCmds {
    Create(WifiGuestCreateArgs),
    Delete(WifiGuestArgs),
    List,
    Get(WifiGuestArgs)
}

#[derive(Args, Debug, Clone)]
pub struct WifiGuestArgs {
    pub name: String,
}

#[derive(Args, Debug, Clone)]
pub struct WifiGuestCreateArgs {
    pub name: String,
    pub key: String,
    #[arg(long, help = "Durée du wifi", default_value_t = 86400)]
    pub duration: i32,
    #[arg(long, help = "Nombre maximum d'utilisations", default_value_t = 100)]
    pub max_use: i32,
    #[arg(value_enum, long, help = "Accès au wifi", default_value_t = WifiGuestAccessType::Network)]
    pub access: WifiGuestAccessType,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum WifiGuestAccessType {
    Full,
    Network
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

    },
    #[clap(about = "Configuration du File Transfer Protocol")]
    Ftp {
        #[clap(subcommand)]
        cmd: FtpCmds,
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum FtpCmds {
    #[clap(alias = "update", about = "Mettre à jour les info FTP")]
    Set(FtpSetArgs),
    #[clap(alias = "info", about = "Récupérer les info FTP")]
    Get(FtpGetArgs)
}

#[derive(Args, Debug, Clone)]
pub struct FtpGetArgs {
    
}

#[derive(Args, Debug, Clone)]
#[group(required = true)]
pub struct FtpSetArgs {
    #[arg(short, long, help = "Activer/désactiver le serveur FTP")]
    pub enable: Option<bool>,
    #[arg(short, long, help = "Mettre à jour le mot de passe du serveur FTP")]
    pub password: Option<String>,
    #[arg(long = "anonymous", help = "Activer/désactiver l'accès anonyme")]
    pub anonymous: Option<FtpAnonymousMode>,
    #[arg(long = "remote", help = "Activer/désactiver l'accès a distance")]
    pub allow_remote: Option<bool>,
    #[arg(long, help = "Mettre à jour le port du controle à distance du serveur FTP")]
    pub remote_port: Option<i32>,
    #[arg(long, help = "Mettre à jour le port des données du controle à distance du serveur FTP")]
    pub remote_port_data: Option<i32>,
    #[arg(long, help = "Mettre à jour le domain du controle à distance du serveur FTP")]
    pub remote_domain: Option<String>,
}

#[derive(ValueEnum, Debug, Clone, PartialEq)]
pub enum FtpAnonymousMode {
    #[clap(name = "off", aliases = ["false"])]
    Off,
    #[clap(name = "on", aliases = ["read"])]
    Read,
    #[clap(name = "write", aliases = ["full"])]
    ReadWrite,
}

#[derive(Subcommand, Debug, Clone)]
pub enum DhcpCmds {
    Get(DhcpGetArgs),
    Set(DhcpSetArgs),
    Lease {
        #[clap(subcommand)]
        cmd: DhcpLeaseCmds,
    },
    Dns {
        #[clap(subcommand)]
        cmd: DhcpDnsCmds,
    }
}

#[derive(Args, Debug, Clone)]
pub struct DhcpGetArgs {
    #[arg(short, long, help = "Configuration ipv6 DHCP", default_value = None, action = ArgAction::SetTrue)]
    pub ipv6: Option<bool>,
}

#[derive(Args, Debug, Clone)]
#[group(required = true)]
pub struct DhcpSetArgs {
    #[arg(short, long, help = "Configuration ipv6 DHCP", default_value = None, action = ArgAction::SetTrue)]
    pub ipv6: Option<bool>,
    pub enable: Option<bool>,
    pub sticky: Option<bool>,
    pub ip_start: Option<String>,
    pub ip_end: Option<String>,
    pub broadcast: Option<bool>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum DhcpLeaseCmds {
    List(ListLeaseArgs),
    Get(GetLeaseArgs),
    Delete(DeleteLeaseArgs),
    Update(UpdateLeaseArgs),
    Create(CreateLeaseArgs),
}

#[derive(Subcommand, Debug, Clone)]
pub enum DhcpDnsCmds {
    #[clap(aliases = ["get", "info"])]
    List(DhcpIpv6Args),
    Add(DhcpDnsArgs),
    Set(DhcpDnsArgs),
    #[clap(aliases = ["delete"])]
    Remove(DhcpDnsArgs),
    Clear(DhcpIpv6Args),
}

#[derive(Args, Debug, Clone)]
pub struct DhcpIpv6Args {
    #[arg(short, long, help = "Configuration ipv6 DNS", default_value = None, action = ArgAction::SetTrue)]
    pub ipv6: Option<bool>
}

#[derive(Args, Debug, Clone)]
pub struct DhcpDnsArgs {
    #[arg(value_delimiter = ' ', required = true)]
    pub dns: Vec<String>,
    #[arg(short, long, help = "Configuration ipv6 DNS", default_value = None, action = ArgAction::SetTrue)]
    pub ipv6: Option<bool>
}

#[derive(Args, Debug, Clone)]
pub struct ListLeaseArgs {
    #[arg(short, long, help = "Affiche les leases dynamiques", default_value = None, action = ArgAction::SetTrue)]
    pub dynamic: Option<bool>
}

#[derive(Args, Debug, Clone)]
pub struct DeleteLeaseArgs {
    pub lease: String,
}

#[derive(Args, Debug, Clone)]
pub struct UpdateLeaseArgs {
    pub lease: String,
    pub comment: Option<String>,
    pub ip: Option<String>,
    pub mac: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct CreateLeaseArgs {
    pub lease: String,
    pub ip: String,
    pub mac: String,
    pub comment: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct GetLeaseArgs {
    pub lease: String,
}