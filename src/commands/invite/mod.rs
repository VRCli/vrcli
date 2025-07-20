mod handlers;

use crate::common::auth_client::AuthenticatedClient;
use anyhow::Result;
use vrcli::InviteAction;

pub async fn handle_invite_command(action: InviteAction) -> Result<()> {
    let auth_client = AuthenticatedClient::new().await?;
    let api_config = auth_client.api_config();

    match action {
        InviteAction::Send {
            user,
            instance_id,
            id,
            request_invite,
            message_slot,
        } => {
            handlers::handle_invite_send_action(
                api_config,
                &user,
                instance_id,
                id,
                request_invite,
                message_slot,
            )
            .await
        }
    }
}
