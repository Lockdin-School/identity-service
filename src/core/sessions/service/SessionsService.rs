use crate::core::sessions::models::SessionConnectionRequest::LiveKitConnectionRequest;
use livekit_api::access_token;
use std::env;

pub struct SessionService {}

impl SessionService {
    pub async fn room_token(
        &self,
        body: LiveKitConnectionRequest,
    ) -> Result<String, access_token::AccessTokenError> {
        let api_key = env::var("LIVEKIT_API_KEY").expect("LIVEKIT_API_KEY is not set");
        let api_secret = env::var("LIVEKIT_API_SECRET").expect("LIVEKIT_API_SECRET is not set");

        let mut token = access_token::AccessToken::with_api_key(&api_key, &api_secret);

        // If this room doesn't exist, it'll be automatically created when
        // the first participant joins
        let room_name = body.room_name;
        token = token.with_grants(access_token::VideoGrants {
            room_join: true,
            room: room_name,
            ..Default::default()
        });

        if let Some(room_config) = body.room_config {
            token = token.with_room_config(room_config);
        }

        // Participant related fields.
        // `participantIdentity` will be available as LocalParticipant.identity
        // within the livekit-client SDK
        token = token.with_identity(
            body.participant_identity
                .unwrap_or_else(|| "quickstart-identity".to_string())
                .as_str(),
        );
        if let Some(participant_metadata) = body.participant_metadata {
            token = token.with_metadata(participant_metadata.as_str());
        }
        if !body.participant_attributes.is_empty() {
            token = token.with_attributes(body.participant_attributes);
        }

        token.to_jwt()
    }
}
