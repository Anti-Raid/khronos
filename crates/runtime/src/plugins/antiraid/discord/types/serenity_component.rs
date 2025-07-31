use serenity::nonmax::NonMaxU32;
use serde::de::Error as DeError;
use serde::ser::{Serializer};
use serde::{Deserialize, Serialize, Deserializer};
use serde_json::{from_value, Value};
use small_fixed_array::{FixedString, FixedArray};
use serde_json::Map as JsonMap;

use serenity::model::prelude::*;

fn default_true() -> bool {
    true
}

fn deserialize_val<T, E>(val: Value) -> Result<T, E>
where
    T: serde::de::DeserializeOwned,
    E: serde::de::Error,
{
    T::deserialize(val).map_err(serde::de::Error::custom)
}

#[macro_export]
macro_rules! internal_enum_number {
    (
        $(#[$outer:meta])*
        $(#[<default> = $default:literal])?
        $vis:vis enum $Enum:ident {
            $(
                $(#[doc = $doc:literal])*
                $(#[cfg $($cfg:tt)*])?
                $Variant:ident = $value:literal,
            )*
            _ => Unknown($T:ty),
        }
    ) => {
        $(#[$outer])*
        $vis struct $Enum (pub $T);

        $(
            impl Default for $Enum {
                fn default() -> Self {
                    Self($default)
                }
            }
        )?

        #[allow(non_snake_case, non_upper_case_globals)]
        #[allow(clippy::allow_attributes, reason = "Does not always trigger due to macro")]
        #[allow(dead_code)]
        impl $Enum {
            $(
                $(#[doc = $doc])*
                $(#[cfg $($cfg)*])?
                $vis const $Variant: Self = Self($value);
            )*
        }
    };
}

internal_enum_number! {
    /// The type of a component
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
    pub enum ComponentType {
        ActionRow = 1,
        Button = 2,
        StringSelect = 3,
        InputText = 4,
        UserSelect = 5,
        RoleSelect = 6,
        MentionableSelect = 7,
        ChannelSelect = 8,
        Section = 9,
        TextDisplay = 10,
        Thumbnail = 11,
        MediaGallery = 12,
        File = 13,
        Separator = 14,
        Container = 17,
        _ => Unknown(u8),
    }
}

/// Represents Discord components, a part of messages that are usually interactable.
///
/// # Component Versioning
///
/// - When `IS_COMPONENTS_V2` is **not** set, the **only** valid top-level component is
///   [`ActionRow`].
/// - When `IS_COMPONENTS_V2` **is** set, other component types may be used at the top level, but
///   other message limitations are applied.
#[derive(Clone, Debug, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum Component {
    ActionRow(ActionRow),
    Button(Button),
    SelectMenu(SelectMenu),
    Section(Section),
    TextDisplay(TextDisplay),
    Thumbnail(Thumbnail),
    MediaGallery(MediaGallery),
    Separator(Separator),
    File(FileComponent),
    Container(Container),
    Unknown(u8),
}

impl<'de> Deserialize<'de> for Component {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = JsonMap::deserialize(deserializer)?;
        let raw_kind = map.get("type").ok_or_else(|| DeError::missing_field("type"))?.clone();
        let value = Value::from(map);

        match deserialize_val(raw_kind)? {
            ComponentType::ActionRow => from_value(value).map(Component::ActionRow),
            ComponentType::Button => from_value(value).map(Component::Button),
            ComponentType::StringSelect
            | ComponentType::UserSelect
            | ComponentType::RoleSelect
            | ComponentType::MentionableSelect
            | ComponentType::ChannelSelect => {
                from_value(value).map(Component::SelectMenu)
            },
            ComponentType::Section => from_value(value).map(Component::Section),
            ComponentType::TextDisplay => {
                from_value(value).map(Component::TextDisplay)
            },
            ComponentType::MediaGallery => {
                from_value(value).map(Component::MediaGallery)
            },
            ComponentType::Separator => from_value(value).map(Component::Separator),
            ComponentType::File => from_value(value).map(Component::File),
            ComponentType::Container => from_value(value).map(Component::Container),
            ComponentType::Thumbnail => from_value(value).map(Component::Thumbnail),
            ComponentType(i) => Ok(Component::Unknown(i)),
        }
        .map_err(DeError::custom)
    }
}

/// A component that is a container for up to 3 text display components and an accessory.
///
/// [Discord docs](https://discord.com/developers/docs/components/reference#section)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Section {
    /// Always [`ComponentType::Section`]
    #[serde(rename = "type")]
    pub kind: ComponentType,
    /// The components inside of the section.
    ///
    /// As of 2025-02-28, this is limited to just [`ComponentType::TextDisplay`] with up to 3 max.
    pub components: FixedArray<Component>,
    /// The accessory to the side of the section.
    ///
    /// As of 2025-02-28, this is limited to [`ComponentType::Button`] or
    /// [`ComponentType::Thumbnail`]
    pub accessory: Box<Component>,
}

/// A section component's thumbnail.
///
/// See [`Section`] for how this fits within a section.
///
/// [Discord docs](https://discord.com/developers/docs/components/reference#thumbnail)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Thumbnail {
    /// Always [`ComponentType::Thumbnail`]
    #[serde(rename = "type")]
    pub kind: ComponentType,
    /// The internal media item this contains.
    pub media: UnfurledMediaItem,
    /// The description of the thumbnail.
    pub description: Option<FixedString<u16>>,
    /// Whether or not this component is spoilered.
    pub spoiler: Option<bool>,
}

/// A url or attachment.
///
/// [Discord docs](https://discord.com/developers/docs/components/reference#unfurled-media-item-structure)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct UnfurledMediaItem {
    /// The url of this item.
    pub url: FixedString<u16>,
    /// The proxied discord url.
    pub proxy_url: Option<FixedString<u16>>,
    /// The width of the media item.
    pub width: Option<NonMaxU32>,
    /// The height of the media item.
    pub height: Option<NonMaxU32>,
    /// The content type of the media item.
    pub content_type: Option<FixedString>,
}

/// A component that allows you to add text to your message, similiar to the `content` field of a
/// message.
///
/// [Discord docs](https://discord.com/developers/docs/components/reference#text-display)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct TextDisplay {
    /// Always [`ComponentType::TextDisplay`]
    #[serde(rename = "type")]
    pub kind: ComponentType,
    /// The content of this text display component.
    pub content: FixedString<u16>,
}

/// A Media Gallery is a component that allows you to display media attachments in an organized
/// gallery format.
///
/// [Discord docs](https://discord.com/developers/docs/components/reference#media-gallery)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct MediaGallery {
    /// Always [`ComponentType::MediaGallery`]
    #[serde(rename = "type")]
    pub kind: ComponentType,
    /// Array of images this media gallery can contain, max of 10.
    pub items: FixedArray<MediaGalleryItem>,
}

/// An individual media gallery item.
///
/// Belongs to [`MediaGallery`].
///
/// [Discord docs](https://discord.com/developers/docs/components/reference#media-gallery-media-gallery-item-structure)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct MediaGalleryItem {
    /// The internal media piece that this item contains.
    pub media: UnfurledMediaItem,
    /// The description of the media item.
    pub description: Option<FixedString<u16>>,
    /// Whether or not this component is spoilered.
    pub spoiler: Option<bool>,
}

/// A component that adds vertical padding and visual division between other components.
///
/// [Discord docs](https://discord.com/developers/docs/components/reference#separator)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Separator {
    /// Always [`ComponentType::Separator`]
    #[serde(rename = "type")]
    pub kind: ComponentType,
    /// Whether or not this contains a separating divider.
    pub divider: Option<bool>,
    /// The spacing of the separator.
    pub spacing: Option<SeparatorSpacingSize>,
}

internal_enum_number! {
    /// The size of a separator component.
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
    #[non_exhaustive]
    pub enum SeparatorSpacingSize {
        Small = 1,
        Large = 2,
        _ => Unknown(u8),
    }
}

/// A file component, will not render a text preview to the user.
///
/// [Discord docs](https://discord.com/developers/docs/components/reference#file)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct FileComponent {
    /// Always [`ComponentType::File`]
    #[serde(rename = "type")]
    pub kind: ComponentType,
    /// The file this component internally contains.
    pub file: UnfurledMediaItem,
    /// Whether or not this component is spoilered.
    pub spoiler: Option<bool>,
}

/// A container component, similar to an embed but without all the functionality.
///
/// [Discord docs](https://discord.com/developers/docs/components/reference#container)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Container {
    /// Always [`ComponentType::Container`]
    #[serde(rename = "type")]
    pub kind: ComponentType,
    /// The accent colour, similar to an embeds accent.
    pub accent_color: Option<Colour>,
    /// Whether or not this component is spoilered.
    pub spoiler: Option<bool>,
    /// The components within this container.
    ///
    /// As of 2025-02-28, this can be [`ComponentType::ActionRow`], [`ComponentType::Section`],
    /// [`ComponentType::TextDisplay`], [`ComponentType::MediaGallery`], [`ComponentType::File`] or
    /// [`ComponentType::Separator`]
    pub components: FixedArray<Component>,
}

/// An action row.
///
/// [Discord docs](https://discord.com/developers/docs/interactions/message-components#action-rows).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct ActionRow {
    /// Always [`ComponentType::ActionRow`]
    #[serde(rename = "type")]
    pub kind: ComponentType,
    /// The components of this ActionRow.
    #[serde(default)]
    pub components: Vec<ActionRowComponent>,
}

/// A component which can be inside of an [`ActionRow`].
///
/// [Discord docs](https://discord.com/developers/docs/interactions/message-components#component-object-component-types).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ActionRowComponent {
    Button(Button),
    SelectMenu(SelectMenu),
    InputText(InputText),
}

impl<'de> Deserialize<'de> for ActionRowComponent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let map = JsonMap::deserialize(deserializer)?;
        let raw_kind = map.get("type").ok_or_else(|| DeError::missing_field("type"))?.clone();
        let value = Value::from(map);

        match deserialize_val(raw_kind)? {
            ComponentType::Button => {
                from_value(value).map(ActionRowComponent::Button)
            },
            ComponentType::InputText => {
                from_value(value).map(ActionRowComponent::InputText)
            },
            ComponentType::StringSelect
            | ComponentType::UserSelect
            | ComponentType::RoleSelect
            | ComponentType::MentionableSelect
            | ComponentType::ChannelSelect => {
                from_value(value).map(ActionRowComponent::SelectMenu)
            },
            ComponentType::ActionRow => {
                return Err(DeError::custom("Invalid component type ActionRow"));
            },
            ComponentType(i) => {
                return Err(DeError::custom(format_args!("Unknown component type {i}")));
            },
        }
        .map_err(DeError::custom)
    }
}

impl Serialize for ActionRowComponent {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        match self {
            Self::Button(c) => c.serialize(serializer),
            Self::InputText(c) => c.serialize(serializer),
            Self::SelectMenu(c) => c.serialize(serializer),
        }
    }
}

impl From<Button> for ActionRowComponent {
    fn from(component: Button) -> Self {
        ActionRowComponent::Button(component)
    }
}

impl From<SelectMenu> for ActionRowComponent {
    fn from(component: SelectMenu) -> Self {
        ActionRowComponent::SelectMenu(component)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ButtonKind {
    Link { url: FixedString },
    Premium { sku_id: SkuId },
    NonLink { custom_id: FixedString, style: ButtonStyle },
}

impl Serialize for ButtonKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a> {
            style: u8,
            #[serde(skip_serializing_if = "Option::is_none")]
            url: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            custom_id: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            sku_id: Option<SkuId>,
        }

        let helper = match self {
            ButtonKind::Link {
                url,
            } => Helper {
                style: 5,
                url: Some(url),
                custom_id: None,
                sku_id: None,
            },
            ButtonKind::Premium {
                sku_id,
            } => Helper {
                style: 6,
                url: None,
                custom_id: None,
                sku_id: Some(*sku_id),
            },
            ButtonKind::NonLink {
                custom_id,
                style,
            } => Helper {
                style: style.0,
                url: None,
                custom_id: Some(custom_id),
                sku_id: None,
            },
        };
        helper.serialize(serializer)
    }
}

/// A button component.
///
/// [Discord docs](https://discord.com/developers/docs/interactions/message-components#button-object-button-structure).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[non_exhaustive]
pub struct Button {
    /// The component type, it will always be [`ComponentType::Button`].
    #[serde(rename = "type")]
    pub kind: ComponentType,
    /// The button kind and style.
    #[serde(flatten)]
    pub data: ButtonKind,
    /// The text which appears on the button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<FixedString>,
    /// The emoji of this button, if there is one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<ReactionType>,
    /// Whether the button is disabled.
    #[serde(default)]
    pub disabled: bool,
}

internal_enum_number! {
    /// The style of a button.
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
    #[non_exhaustive]
    pub enum ButtonStyle {
        Primary = 1,
        Secondary = 2,
        Success = 3,
        Danger = 4,
        // No Link, because we represent Link using enum variants
        _ => Unknown(u8),
    }
}

/// A select menu component.
///
/// [Discord docs](https://discord.com/developers/docs/interactions/message-components#select-menu-object-select-menu-structure).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct SelectMenu {
    /// The component type, which may either be [`ComponentType::StringSelect`],
    /// [`ComponentType::UserSelect`], [`ComponentType::RoleSelect`],
    /// [`ComponentType::MentionableSelect`], or [`ComponentType::ChannelSelect`].
    #[serde(rename = "type")]
    pub kind: ComponentType,
    /// An identifier defined by the developer for the select menu.
    pub custom_id: Option<FixedString>,
    /// The options of this select menu.
    ///
    /// Required for [`ComponentType::StringSelect`] and unavailable for all others.
    #[serde(default)]
    pub options: FixedArray<SelectMenuOption>,
    /// List of channel types to include in the [`ComponentType::ChannelSelect`].
    #[serde(default)]
    pub channel_types: FixedArray<ChannelType>,
    /// The placeholder shown when nothing is selected.
    pub placeholder: Option<FixedString>,
    /// The minimum number of selections allowed.
    pub min_values: Option<u8>,
    /// The maximum number of selections allowed.
    pub max_values: Option<u8>,
    /// Whether select menu is disabled.
    #[serde(default)]
    pub disabled: bool,
}

/// A select menu component options.
///
/// [Discord docs](https://discord.com/developers/docs/interactions/message-components#select-menu-object-select-option-structure).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct SelectMenuOption {
    /// The text displayed on this option.
    pub label: FixedString,
    /// The value to be sent for this option.
    pub value: FixedString,
    /// The description shown for this option.
    pub description: Option<FixedString>,
    /// The emoji displayed on this option.
    pub emoji: Option<ReactionType>,
    /// Render this option as the default selection.
    #[serde(default)]
    pub default: bool,
}

/// An input text component for modal interactions
///
/// [Discord docs](https://discord.com/developers/docs/interactions/message-components#text-inputs-text-input-structure).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct InputText {
    /// The component type, it will always be [`ComponentType::InputText`].
    #[serde(rename = "type")]
    pub kind: ComponentType,
    /// Developer-defined identifier for the input; max 100 characters
    pub custom_id: FixedString<u16>,
    /// The [`InputTextStyle`]. Required when sending modal data.
    ///
    /// Discord docs are wrong here; it says the field is always sent in modal submit interactions
    /// but it's not. It's only required when _sending_ modal data to Discord.
    /// <https://github.com/discord/discord-api-docs/issues/6141>
    pub style: Option<InputTextStyle>,
    /// Label for this component; max 45 characters. Required when sending modal data.
    ///
    /// Discord docs are wrong here; it says the field is always sent in modal submit interactions
    /// but it's not. It's only required when _sending_ modal data to Discord.
    /// <https://github.com/discord/discord-api-docs/issues/6141>
    pub label: Option<FixedString<u8>>,
    /// Minimum input length for a text input; min 0, max 4000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u16>,
    /// Maximum input length for a text input; min 1, max 4000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u16>,
    /// Whether this component is required to be filled (defaults to true)
    #[serde(default = "default_true")]
    pub required: bool,
    /// When sending: Pre-filled value for this component; max 4000 characters (may be None).
    ///
    /// When receiving: The input from the user (always Some)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<FixedString<u16>>,
    /// Custom placeholder text if the input is empty; max 100 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<FixedString<u16>>,
}

internal_enum_number! {
    /// The style of the input text
    ///
    /// [Discord docs](https://discord.com/developers/docs/interactions/message-components#text-inputs-text-input-styles).
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
    #[non_exhaustive]
    pub enum InputTextStyle {
        Short = 1,
        Paragraph = 2,
        _ => Unknown(u8),
    }
}