/// Returns the highest role of a member in a guild.
pub fn highest_role<'a>(
    guild: &'a serenity::all::PartialGuild,
    member: &serenity::all::Member,
) -> Option<&'a serenity::all::Role> {
    let mut highest_role: Option<&serenity::all::Role> = None;
    for role_id in &member.roles {
        if let Some(role) = guild.roles.get(role_id) {
            if let Some(highest_role_obj) = &highest_role {
                if role > highest_role_obj {
                    highest_role = Some(role);
                }
            } else {
                highest_role = Some(role);
            }
        }
    }

    highest_role
}
