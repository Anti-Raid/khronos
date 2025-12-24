use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetInvite {
    pub code: String,
    pub with_counts: bool,
    pub with_expiration: bool,
    pub guild_scheduled_event_id: Option<serenity::all::ScheduledEventId>,
}

impl ApiReq for GetInvite {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let invite = this.controller()
            .get_invite(&self.code, self.with_counts, self.with_expiration, self.guild_scheduled_event_id)
            .await?;

        Ok(invite)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetInvite(self)
    }
}
