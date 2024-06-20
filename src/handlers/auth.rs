use std::collections::HashMap;
use crate::app::{App};
use crate::config::{FbxSession};
use crate::models::args::{AuthLoginArgs, AuthSetUrlArgs};
use crate::models::exception::ClientError;
use crate::models::freebox::authorization::{AuthSessionStartRequest, AuthTokenRequest, AuthTokenResult, AuthTrackAuthorizationProgressStatus};
use crate::services::api::{FreeboxOSCalls, LoginCalls};
use url::Url;
use crate::terminal::{handler_result, HandlerResult};

pub const HEADER_FBX_TOKEN: &str = "X-Fbx-App-Auth";

pub struct Auth;

impl Auth {
    pub async fn login(app: &mut App, arg: AuthLoginArgs) -> HandlerResult {
        let url = Url::parse(arg.url.as_str())
            .map_err(|_e| ClientError::InvalidUrl("Url non valide !"))?;
        app.client.set_url(url.to_string());

        let token_request = AuthTokenRequest::new(&arg.app_id, &arg.app_version);
        let token_result = if let Some(fbx_app) = &app.config.app {
            match &fbx_app.app_token {
                Some(x) => AuthTokenResult::new(x.clone(), fbx_app.track_id),
                None => Self::register_app(app, &token_request).await?
            }
        } else {
            Self::register_app(app, &token_request).await?
        };
        loop {
            let response = app
                .api
                .get_authorization_status(&app.client, token_result.track_id)
                .await?;
            match response.result.unwrap().status {
                AuthTrackAuthorizationProgressStatus::Timeout => {
                    println!("Timeout de l'acceptation de l'application sur la Freebox...");
                    break;
                },
                AuthTrackAuthorizationProgressStatus::Denied => {
                    println!("L'application a été refusée sur la Freebox...");
                    break;
                },
                AuthTrackAuthorizationProgressStatus::Granted => {
                    app.config.register_app_granted();
                    break;
                },
                _ => {}
            };
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
        let login_response = app.api.login(&app.client, None).await?;
        let body = AuthSessionStartRequest::new(
            token_request.clone(),
            token_result.app_token,
            login_response.result.unwrap().challenge.expect(""),
        )
        .ok_or(ClientError::RequestError("Token"))?;
        let response = app.api.start_session(&app.client, body).await?;
        if response.success {
            app.config.session = Some(FbxSession {
                token_session: response.result.session_token,
                created_at: chrono::Utc::now(),
            });
            app.config.pref.base_url = url.to_string();
            println!("Authentification réussie !");
        } else {
            println!("Authentification échouée !");
        }
        Err(ClientError::Unauthorized(""))
    }

    pub async fn register_app(app: &mut App, token_request: &AuthTokenRequest) -> Result<AuthTokenResult, ClientError> {
        let app_register_response = app
            .api
            .authorize_app(&app.client, token_request.clone())
            .await?;
        app.config.register_app_pending(
            &token_request.app_id,
            &token_request.app_version,
            &app_register_response,
        );
        Ok(app_register_response.result)
    }

    pub async fn status(_app: &mut App) -> HandlerResult {
        if _app.config.app.is_none() {
            Err(ClientError::Unauthorized("You must be logged in"))?;
        }
        if _app.config.session.is_none() {
            Err(ClientError::Unauthorized("You must be logged in"))?;
        }
        match &_app.config.session {
            Some(session) => {
                let login_response = _app
                    .api
                    .login(&_app.client, Some(session.token_session.as_str()))
                    .await?;
                match login_response.result.unwrap().logged_in {
                    true => println!("Logged in !"),
                    false => println!("Not logged in !"),
                }
            }
            None => println!("Session not initialized"),
        }
        Err(ClientError::Unauthorized("You must be logged in"))
    }

    pub async fn logout(app: &mut App) -> HandlerResult {
        let session = required_login(app).await?;
        handler_result(app.api.logout(&app.client, &session)).await
    }

    pub async fn set_url(app: &mut App, args: AuthSetUrlArgs) -> HandlerResult {
        let url = Url::parse(args.url.as_str())
            .map_err(|_e| ClientError::InvalidUrl("Url non valide !"))?;
        app.client.set_url(url.to_string());
        if !args.skip_verification {
            app.api.api_version(&app.client).await?;
        }
        app.config.pref.base_url = url.to_string();
        Err(ClientError::Unauthorized("TODO: OK"))
    }

    pub async fn init(_app: &mut App) -> Result<(), ClientError> {
        Ok(())
    }
}

pub async fn required_login(app: &mut App) -> Result<String, ClientError> {
    match &app.config.session {
        Some(session) => {
            match app.api.login(&app.client, Some(session.token_session.as_str())).await?.result.unwrap().logged_in {
                true => Ok(session.token_session.clone()),
                false => Ok(renew_session(app).await?),
            }
        },
        None => Err(ClientError::Unauthorized("You must be logged in")),
    }
}

pub async fn renew_session(app: &mut App) -> Result<String, ClientError> {
    match &app.config.app {
        Some(fbx_app) => {
            let token_result = AuthTokenResult::new(fbx_app.app_token.clone().ok_or(ClientError::Unauthorized("You must be logged in"))?, fbx_app.track_id);
            let login_response = app.api.login(&app.client, None).await?;
            let body = AuthSessionStartRequest::new(
                AuthTokenRequest::new(&Some(fbx_app.app_id.clone()), &Some(fbx_app.version.clone())),
                token_result.app_token,
                login_response.result.unwrap().challenge.ok_or(ClientError::RequestError("You must be logged in"))?,
            )
            .ok_or(ClientError::RequestError("Token"))?;
            let response = app.api.start_session(&app.client, body).await?;
            if response.success {
                app.config.session = Some(FbxSession {
                    token_session: response.result.session_token.clone(),
                    created_at: chrono::Utc::now(),
                });
                Ok(response.result.session_token)
            } else {
                Err(ClientError::Unauthorized("You must be logged in"))
            }
        },
        None => Err(ClientError::Unauthorized("You must be logged in")),
    }
}

pub fn auth_header(session_token: &str) -> Option<HashMap<String, &str>> {
    Some(HashMap::from([(String::from(HEADER_FBX_TOKEN), session_token)]))
}