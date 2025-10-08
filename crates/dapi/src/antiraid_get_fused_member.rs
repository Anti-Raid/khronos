use crate::{ApiReq, context::{AntiraidFusedMember, DiscordContext}, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AntiRaidGetFusedMember {
    pub ids: Vec<String>,
}

impl ApiReq for AntiRaidGetFusedMember {
    type Resp = AntiraidFusedMember;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let mut user_ids = Vec::with_capacity(self.ids.len());

        for user_id in self.ids {
            let user_id: serenity::all::UserId = user_id
                .parse()?;
            user_ids.push(user_id);
        }

        let fused_member = this.get_fused_member(user_ids).await?;

        Ok(fused_member)
    }
}
