use crate::{context::{AntiraidFusedMember, DiscordContext}, controller::DiscordProvider};

pub async fn antiraid_get_fused_member<T: DiscordProvider>(this: &DiscordContext<T>, ids: Vec<String>) -> Result<AntiraidFusedMember, crate::Error> {
    let mut user_ids = Vec::with_capacity(ids.len());

    for user_id in ids {
        let user_id: serenity::all::UserId = user_id
            .parse()?;
        user_ids.push(user_id);
    }

    let fused_member = this.get_fused_member(user_ids).await?;

    Ok(fused_member)
}