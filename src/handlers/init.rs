/*use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use spinners::{Spinner, Spinners};
use crate::app::App;
use crate::{config, models};
use crate::models::args::InitArgs;
use crate::models::exception::ApiException;
use crate::services::api::{FreeboxOSApi, FreeboxOSApiCalls};
use crate::services::discover::{Device, Discover, FreeboxDiscover};

impl InitHandler for Init {
    async fn handle(app: &mut App, args: &InitArgs) -> Result<(), ApiException>{
        /*if args.discover && args.url.is_none() {
            let mut sp = Spinner::new(Spinners::Line, "Discovering freebox devices...".into());
            let a = FreeboxDiscover.discover_device(Duration::from_secs(args.time)).await;
            let b = a.await;
            sp.stop_with_newline();
            for x in b {
                println!("{:?}", x.get_info(vec![]));
            }
        } else {
            match &args.url {
                Some(url) => {
                    println!("URL provided: {:?}", url);
                    let client = FreeboxOSApi::new(url.clone());
                    let response = client.authorize_app(models::freebox::authorization::AuthTokenRequest {
                        app_id: "dev.nikho.fbxcli".to_string(),
                        app_name: "fbx-cli".to_string(),
                        app_version: "1.0.0".to_string(),
                        device_name: "My device".to_string()
                    }).await.unwrap();

                    app.config.app = Some(config::FbxApp {
                        track_id: response.result.track_id.clone(),
                        app_id: "dev.nikho.fbxcli".to_string(),
                        status: config::FbxAppStatus::Pending,
                        date: chrono::Utc::now(),
                        version: "1.0.0".to_string()
                    });
                    app.config.save().unwrap();

                    let a = client.get_authorization_status(response.result.track_id).await.unwrap();
                    println!("Challenge: {:?}", a);
                    print!("En attente de l'accepation de l'application sur la freebox..");
                    loop {
                        sleep(Duration::from_secs(2));
                        print!(".");
                        stdout().flush().unwrap();
                        let a = client.get_authorization_status(response.result.track_id).await.unwrap();
                        if a.result.status == models::freebox::authorization::AuthTrackAuthorizationProgressStatus::Granted {
                            println!("Application accepted ! {:?}", a);
                            break;
                        }
                    }
                    println!("Application accepted !");
                    println!("Good hacking :)");

                    let log_res = client.login(None).await.unwrap();
                    let mut hasher: Hmac<Sha1> = Mac::new_from_slice(response.result.app_token.as_bytes()) .expect("HMAC algoritms can take keys of any size");
                    hasher.update(log_res.result.challenge.expect("Challenge cannot be null").as_bytes());
                    let a = hasher.finalize();
                    let b = a.into_bytes().iter().map(|b| format!("{:02x}", b)).collect::<String>();
                    println!("HMAC SHA-1: {:?}", b);
                    let a = client.start_session(models::freebox::authorization::AuthSessionStartRequest {
                        app_id: "dev.nikho.fbxcli".to_string(),
                        password: b,
                        app_version: "1.0.0".to_string()
                    }).await.unwrap();
                    println!("Session started: {:?}", a);
                    app.config.app = Some(config::FbxApp {
                        track_id: response.result.track_id.clone(),
                        app_id: "dev.nikho.fbxcli".to_string(),
                        status: config::FbxAppStatus::Granted,
                        date: chrono::Utc::now(),
                        version: "1.0.0".to_string()
                    });
                    app.config.session = Some(config::FbxSession {
                        app_id: "dev.nikho.fbxcli".to_string(),
                        app_token: response.result.app_token.clone(),
                        token: a.result.session_token.clone(),
                        created_at: chrono::Utc::now()
                    });
                },
                None => {
                    println!("No url provided");
                }

            }
        }*/
        Ok(())
    }
}*/