use antiraid_types::ar_event::AntiraidEvent;
use khronos_runtime::{primitives::event::CreateEvent, Error};
use serenity::all::{Context, FullEvent, GuildId, Interaction};

#[allow(dead_code)]
pub const RESERVED_COMMAND_NAMES: &[&str] = &[
    "help",
    "stats",
    "ping",
    "whois",
    "modules",
    "commands",
    "web",
    "lockdowns",
    "moderation",
    "backups",
];

#[allow(dead_code)]
#[inline]
const fn not_audit_loggable_event() -> &'static [&'static str] {
    &[
        "CACHE_READY",         // Internal
        "GUILD_CREATE",        // Internal
        "GUILD_MEMBERS_CHUNK", // Internal
    ]
}

#[allow(dead_code)]
pub async fn parse_discord_event(
    event: &FullEvent,
    serenity_context: &Context,
) -> Result<Option<(CreateEvent, GuildId)>, Error> {
    let Some(guild_id) = gwevent::core::get_event_guild_id(event) else {
        return Ok(None);
    };

    let event_snake_name = event.into();
    if not_audit_loggable_event().contains(&event_snake_name) {
        return Ok(None);
    }

    let user_id = gwevent::core::get_event_user_id(event);

    let event_data = match event {
        FullEvent::GuildAuditLogEntryCreate { .. } => serde_json::to_value(event)?,
        FullEvent::InteractionCreate { interaction } => {
            match interaction {
                Interaction::Ping(_) => return Ok(None),
                Interaction::Command(i) | Interaction::Autocomplete(i) => {
                    if RESERVED_COMMAND_NAMES.contains(&i.data.name.as_str()) {
                        return Ok(None);
                    }

                    let mut value = serde_json::to_value(interaction)?;

                    // Inject in type
                    if let serde_json::Value::Object(ref mut map) = value {
                        let typ: u8 = serenity::all::InteractionType::Command.0;
                        map.insert("type".to_string(), serde_json::Value::Number(typ.into()));
                    }

                    serde_json::json!({
                        "InteractionCreate": {
                            "interaction": value
                        }
                    })
                }
                _ => {
                    let mut value = serde_json::to_value(interaction)?; // Allow Component+Modal interactions to freely passed through

                    // Inject in type
                    if let serde_json::Value::Object(ref mut map) = value {
                        let typ: u8 = interaction.kind().0;
                        map.insert("type".to_string(), serde_json::Value::Number(typ.into()));
                    }

                    serde_json::json!({
                        "InteractionCreate": {
                            "interaction": value
                        }
                    })
                }
            }
        }
        _ => {
            if let Some(user_id) = user_id {
                if user_id == serenity_context.cache.current_user().id {
                    return Ok(None);
                }
            }

            serde_json::to_value(event)?
        }
    };
    
    let event_name: &'static str = event_snake_name.into();

    Ok(Some((
        CreateEvent::new(
            "Discord".to_string(),
            event_name.to_uppercase(),
            event_data,
            user_id.map(|u| u.to_string()),
        ),
        guild_id,
    )))
}

#[allow(dead_code)]
/// Parses an antiraid event into a template event
pub fn parse_event(event: &AntiraidEvent) -> Result<CreateEvent, Error> {
    Ok(CreateEvent::new(
        "AntiRaid".to_string(),
        event.to_string(),
        event.to_value()?,
        event.author(),
    ))
}
