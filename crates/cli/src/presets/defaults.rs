use antiraid_types::userinfo::UserInfo;
use extract_map::ExtractMap;
use kittycat::perms::StaffPermissions;
use serenity::all::Permissions;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    LazyLock,
};

pub(crate) fn default_global_user_id() -> serenity::all::UserId {
    crate::constants::default_global_user_id()
}

pub(crate) fn default_global_user() -> serenity::all::User {
    let mut default = serenity::all::User::default();
    default.id = default_global_user_id();
    default
}

pub(crate) fn default_global_member() -> serenity::all::Member {
    let mut default = serenity::all::Member::default();
    default.user = default_global_user();
    default.guild_id = default_global_guild_id();
    default
}

pub(crate) fn default_global_guild_id() -> serenity::all::GuildId {
    crate::constants::default_global_guild_id()
}

pub(crate) fn default_global_unknown_string() -> String {
    "unknown".to_string()
}

pub(crate) fn default_global_userinfo() -> UserInfo {
    UserInfo {
        discord_permissions: Permissions::MANAGE_GUILD | Permissions::ADMINISTRATOR,
        kittycat_staff_permissions: StaffPermissions {
            user_positions: Vec::new(),
            perm_overrides: Vec::new(),
        },
        guild_owner_id: default_global_user_id(),
        guild_roles: ExtractMap::default(),
        member_roles: Vec::new(),
        kittycat_resolved_permissions: Vec::new(),
    }
}

static CURRENT_CORRELATION_ID: LazyLock<AtomicUsize> = LazyLock::new(|| AtomicUsize::new(1));

fn make_uuid_from_int(id: usize) -> uuid::Uuid {
    let mut bytes = [0u8; 16];
    bytes[0..8].copy_from_slice(&id.to_le_bytes());
    uuid::Uuid::from_bytes(bytes)
}

pub(crate) fn default_moderation_start_correlation_id() -> uuid::Uuid {
    let curr_id = CURRENT_CORRELATION_ID.fetch_add(1, Ordering::SeqCst);
    make_uuid_from_int(curr_id)
}

pub(crate) fn default_moderation_end_correlation_id() -> uuid::Uuid {
    let curr_id = CURRENT_CORRELATION_ID.load(Ordering::SeqCst);
    make_uuid_from_int(curr_id - 1) // We want the previous ID
}

#[cfg(test)]
mod test_uuid {
    use super::*;

    #[test]
    fn test_make_uuid_from_int() {
        let mut id = 0;
        let uuid = make_uuid_from_int(id);
        assert_eq!(uuid.to_string(), "00000000-0000-0000-0000-000000000000");

        // Ensure repeated calls do not change the UUID
        let uuid = make_uuid_from_int(id);
        assert_eq!(uuid.to_string(), "00000000-0000-0000-0000-000000000000");

        id += 1;

        let uuid = make_uuid_from_int(id);
        assert_eq!(uuid.to_string(), "01000000-0000-0000-0000-000000000000");

        id += 1;

        let uuid = make_uuid_from_int(id);
        assert_eq!(uuid.to_string(), "02000000-0000-0000-0000-000000000000");
    }

    #[test]
    fn test_default_moderation_start_correlation_id() {
        // Ensure to reset the counter
        CURRENT_CORRELATION_ID.store(1, Ordering::SeqCst);

        let uuid = default_moderation_start_correlation_id();
        assert_eq!(uuid.to_string(), "01000000-0000-0000-0000-000000000000");

        let uuid = default_moderation_start_correlation_id();
        assert_eq!(uuid.to_string(), "02000000-0000-0000-0000-000000000000");
    }

    #[test]
    fn test_default_moderation_end_correlation_id() {
        // Ensure to reset the counter
        CURRENT_CORRELATION_ID.store(1, Ordering::SeqCst);

        // Base cases
        let start_id = default_moderation_start_correlation_id();
        let end_id = default_moderation_end_correlation_id();
        assert_eq!(start_id.to_string(), "01000000-0000-0000-0000-000000000000");
        assert_eq!(start_id, end_id);

        let start_id = default_moderation_start_correlation_id();
        let end_id = default_moderation_end_correlation_id();
        assert_eq!(start_id.to_string(), "02000000-0000-0000-0000-000000000000");
        assert_eq!(start_id, end_id);

        // Ensure to reset the counter
        CURRENT_CORRELATION_ID.store(1, Ordering::SeqCst);

        for i in 1..10 {
            let start_id = default_moderation_start_correlation_id();
            let end_id = default_moderation_end_correlation_id();
            assert_eq!(start_id, make_uuid_from_int(i));
            assert_eq!(start_id, end_id);
        }
    }
}
