use crate::app::App;
use crate::config::{FbxApp, FbxAppStatus, FbxSession};
use crate::models::args::AuthLoginArgs;
use crate::models::exception::ClientError;
use crate::models::freebox::authorization::{
    AuthSessionStartRequest, AuthTokenRequest, AuthTrackAuthorizationProgressStatus,
};
use crate::services::api::FreeboxOSApiCalls;
use url::Url;

pub struct Auth;

impl Auth {
    pub async fn login(app: &mut App, arg: AuthLoginArgs) -> Result<(), ClientError> {
        let url = Url::parse(arg.url.as_str())
            .map_err(|_e| ClientError::InvalidUrl("Url non valide !"))?;
        app.client.set_url(url.to_string());
        let auth_token_request_body = AuthTokenRequest::default();
        let app_register_response = app
            .api
            .authorize_app(&app.client, auth_token_request_body.clone())
            .await?;
        println!("En attente de l'acceptation de l'application sur la Freebox...");
        loop {
            let response = app
                .api
                .get_authorization_status(&app.client, app_register_response.result.track_id)
                .await?;
            if response.result.status == AuthTrackAuthorizationProgressStatus::Granted {
                app.config.app = Some(FbxApp {
                    track_id: app_register_response.result.track_id.clone(),
                    app_id: auth_token_request_body.app_id.clone(),
                    status: FbxAppStatus::Granted(app_register_response.result.app_token.clone()),
                    created_at: chrono::Utc::now(),
                    authorized_at: Some(chrono::Utc::now()),
                    version: auth_token_request_body.app_version.clone(),
                });
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
        let login_response = app.api.login(&app.client, None).await?;
        let body = AuthSessionStartRequest::new(
            app_register_response.result.app_token,
            login_response.result.challenge.expect(""),
        )
        .ok_or_else(|| ClientError::RequestError("Token"))?;
        let response = app.api.start_session(&app.client, body).await?;
        if response.success {
            app.config.session = Some(FbxSession {
                token_session: response.result.session_token,
                created_at: chrono::Utc::now(),
            });
            println!("Authentification réussie !");
        } else {
            println!("Authentification échouée !");
        }
        Ok(())
    }

    pub async fn status(_app: &mut App) -> Result<(), ClientError> {
        if _app.config.app.is_none() {
            println!("Login not initialized");
            return Ok(());
        }
        if _app.config.session.is_none() {
            println!("Session not initialized");
            return Ok(());
        }
        match &_app.config.session {
            Some(session) => {
                println!("Session token: {}", session.token_session);
                let a = _app
                    .api
                    .login(&_app.client, Some(session.token_session.clone()))
                    .await;
                match a {
                    Ok(res) => {
                        println!("{:?}", res);
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            }
            None => {
                println!("Session not initialized");
            }
        }
        Ok(())
    }

    pub async fn logout(_app: &mut App) -> Result<(), ClientError> {
        Ok(())
    }

    pub async fn set_url(_app: &mut App) -> Result<(), ClientError> {
        Ok(())
    }

    pub async fn init(_app: &mut App) -> Result<(), ClientError> {
        Ok(())
    }

    /*async fn handle(app: &mut App, arg: &AuthCommands) -> Result<(), ApiException> {
        match arg {
            AuthCommands::Status => {
                println!("Status auth");
            },
            AuthCommands::Logout => {
                println!("Logout auth");
            },
            AuthCommands::Login(_args) => {
                println!("Login auth");
            },
            AuthCommands::SetUrl(_args) => {
                println!("Url updated !");
            },
            AuthCommands::Init(args) => {
                if let Some(url) = &args.url {
                    app.client.set_url(url.clone());
                }
                //let disco = FreeboxDiscover {};
                //let a = disco.discover_device(Duration::from_secs(5)).await.await;
                //println!("Init auth");
                let log_res = app.api.login(&app.client, None).await.unwrap();
                println!("{:?}", log_res);

                /*let mut hasher: Hmac<Sha1> = Mac::new_from_slice(response.result.app_token.as_bytes()) .expect("HMAC algoritms can take keys of any size");
                hasher.update(log_res.result.challenge.expect("Challenge cannot be null").as_bytes());*/

            }
        }
        Ok(())
    }*/
}
