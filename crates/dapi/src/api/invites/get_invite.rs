use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GetInvite {
    pub code: String,
    pub with_counts: bool,
}

impl ApiReq for GetInvite {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let invite = this.controller()
            .get_invite(&self.code, self.with_counts)
            .await?;

        Ok(invite)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetInvite(self)
    }
}
