use crate::configuration::state::AppState;
use crate::core::sessions::models::SessionConnectionRequest::{
    LiveKitConnectionRequest, LiveKitConnectionResponse,
};
use actix_web::web::Data;
use actix_web::{HttpResponse, post, web};
use tokio::io;
// use crate::infrastructure::errors::handle_auth_error;

#[post("/token")]
pub async fn get_session_token(
    state: Data<AppState>,
    body: web::Json<LiveKitConnectionRequest>,
) -> actix_web::Result<HttpResponse> {
    match state.session_service.room_token(body.into_inner()).await {
        Ok(participant_token) => {
            log::info!(
                "POST /sessions/token - participant_token: {}",
                participant_token
            );
            let server_url = std::env::var("LIVEKIT_URL").expect("LIVEKIT_URL is not set");
            let token_response = LiveKitConnectionResponse {
                server_url,
                participant_token,
            };
            Ok(HttpResponse::Ok().json(token_response))
        }
        Err(e) => {
            log::error!("POST /rooms/token - Room Access Token Error. \n{}", e);
            Ok(HttpResponse::from_error(io::Error::other(e.to_string())))
        }
    }
}
