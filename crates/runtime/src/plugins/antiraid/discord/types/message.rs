use crate::internal_enum_number;

use super::allowed_mentions::CreateAllowedMentions;
use super::attachment::CreateMessageAttachment;
use super::embed::CreateEmbed;
use super::poll::CreatePoll;
use serde::{Deserialize, Serialize};
use serenity::all::*;
use super::serenity_component::Component as SerenityComponent;

/// The macro forwards the generation to the `bitflags::bitflags!` macro and implements the default
/// (de)serialization for Discord's bitmask values.
///
/// The flags are created with `T::from_bits_truncate` for the deserialized integer value.
///
/// Use the `bitflags::bitflags! macro directly if a different serde implementation is required.
#[macro_export]
macro_rules! internal_bitflags {
    (
        $(#[$outer:meta])*
        $vis:vis struct $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Flag:ident = $value:expr;
            )*
        }
    ) => {
        $(#[$outer])*
        #[repr(Rust, packed)]
        $vis struct $BitFlags($T);

        bitflags::bitflags! {
            impl $BitFlags: $T {
                $(
                    $(#[$inner $($args)*])*
                    const $Flag = $value;
                )*
            }
        }

        internal_bitflags!(__impl_serde $BitFlags: $T);
    };
    (__impl_serde $BitFlags:ident: $T:tt) => {
        impl<'de> serde::de::Deserialize<'de> for $BitFlags {
            fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
                Ok(Self::from_bits_truncate(<$T>::deserialize(deserializer)?))
            }
        }

        impl serde::ser::Serialize for $BitFlags {
            fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
                self.bits().serialize(serializer)
            }
        }
    };
}

internal_bitflags! {
    /// Describes extra features of the message.
    ///
    /// [Discord docs](https://discord.com/developers/docs/resources/channel#message-object-message-flags).
    #[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq)]
    pub struct MessageFlags: u64 {
        /// This message has been published to subscribed channels (via Channel Following).
        const CROSSPOSTED = 1 << 0;
        /// This message originated from a message in another channel (via Channel Following).
        const IS_CROSSPOST = 1 << 1;
        /// Do not include any embeds when serializing this message.
        const SUPPRESS_EMBEDS = 1 << 2;
        /// The source message for this crosspost has been deleted (via Channel Following).
        const SOURCE_MESSAGE_DELETED = 1 << 3;
        /// This message came from the urgent message system.
        const URGENT = 1 << 4;
        /// This message has an associated thread, with the same id as the message.
        const HAS_THREAD = 1 << 5;
        /// This message is only visible to the user who invoked the Interaction.
        const EPHEMERAL = 1 << 6;
        /// This message is an Interaction Response and the bot is "thinking".
        const LOADING = 1 << 7;
        /// This message failed to mention some roles and add their members to the thread.
        const FAILED_TO_MENTION_SOME_ROLES_IN_THREAD = 1 << 8;
        /// This message will not trigger push and desktop notifications.
        const SUPPRESS_NOTIFICATIONS = 1 << 12;
        /// This message is a voice message.
        ///
        /// Voice messages have the following properties:
        /// - They cannot be edited.
        /// - Only a single audio attachment is allowed. No content, stickers, etc...
        /// - The [`Attachment`] has additional fields: `duration_secs` and `waveform`.
        ///
        /// As of 2023-04-14, clients upload a 1 channel, 48000 Hz, 32kbps Opus stream in an OGG container.
        /// The encoding is a Discord implementation detail and may change without warning or documentation.
        ///
        /// As of 2023-04-20, bots are currently not able to send voice messages
        /// ([source](https://github.com/discord/discord-api-docs/pull/6082)).
        const IS_VOICE_MESSAGE = 1 << 13;
        /// Enables support for sending Components V2.
        ///
        /// Setting this flag is required to use V2 components.
        /// Attempting to send V2 components without enabling this flag will result in an error.
        ///
        /// # Limitations
        /// When this flag is enabled, certain restrictions apply:
        /// - The `content` and `embeds` fields cannot be set.
        /// - Audio file attachments are not supported.
        /// - Files will not have a simple text preview.
        /// - URLs will not generate embeds.
        ///
        /// For more details, refer to the Discord documentation: [https://discord.com/developers/docs/components/reference#component-reference]
        const IS_COMPONENTS_V2 = 1 << 15;

    }
}

/// [Discord docs](https://discord.com/developers/docs/resources/channel#create-message)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[must_use]
pub struct CreateMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<serde_json::Value>,
    #[serde(default)]
    pub tts: bool,
    #[serde(default)]
    pub embeds: Vec<CreateEmbed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<CreateAllowedMentions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<MessageReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<SerenityComponent>>,
    #[serde(default)]
    pub sticker_ids: Vec<StickerId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    #[serde(default)]
    pub enforce_nonce: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<CreatePoll>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<CreateMessageAttachment>,
}

/// [Discord docs](https://discord.com/developers/docs/resources/channel#edit-message)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[must_use]
pub struct EditMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<CreateEmbed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<CreateAllowedMentions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<SerenityComponent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<CreateMessageAttachment>,
}

internal_enum_number! {
    /// Message Reference Type information
    ///
    /// [Discord docs](https://discord.com/developers/docs/resources/message#message-reference-types)
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
    #[<default> = 0]
    pub enum MessageReferenceKind {
        Default = 0,
        Forward = 1,
        _ => Unknown(u8),
    }
}

/// Reference data sent with crossposted messages.
///
/// [Discord docs](https://discord.com/developers/docs/resources/channel#message-reference-object-message-reference-structure).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct MessageReference {
    /// The Type of Message Reference
    #[serde(rename = "type", default = "MessageReferenceKind::default")]
    pub kind: MessageReferenceKind,
    /// ID of the originating message.
    pub message_id: Option<MessageId>,
    /// ID of the originating message's channel.
    pub channel_id: GenericChannelId,
    /// ID of the originating message's guild.
    pub guild_id: Option<GuildId>,
    /// When sending, whether to error if the referenced message doesn't exist instead of sending
    /// as a normal (non-reply) message, default true.
    pub fail_if_not_exists: Option<bool>,
}
