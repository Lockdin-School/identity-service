use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Incoming payload used to generate a LiveKit token source.
/// Most fields are optional because token generation can be driven by partial client input.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveKitConnectionRequest {
    #[serde(default)]
    pub room_name: String,
    #[serde(default)]
    pub participant_name: Option<String>,
    #[serde(default)]
    pub participant_identity: Option<String>,
    #[serde(default)]
    pub participant_metadata: Option<String>,
    #[serde(default)]
    pub participant_attributes: HashMap<String, String>,
    #[serde(default)]
    pub room_config: Option<livekit_protocol::RoomConfiguration>,
}

/// Minimal response required by the client to connect to LiveKit.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveKitConnectionResponse {
    pub server_url: String,
    pub participant_token: String,
}
