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
            message_slot,
        } => {
            handlers::handle_invite_send_action(
                api_config,
                &user,
                Some(instance_id),
                id,
                false,
                message_slot,
            )
            .await
        }
        InviteAction::Request {
            user,
            id,
            message_slot,
            force_request,
        } => {
            handlers::handle_invite_request_action(
                api_config,
                &user,
                id,
                message_slot,
                force_request,
            )
            .await
        }
    }
}
