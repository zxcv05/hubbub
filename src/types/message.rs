use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::message::embed::Embed;
use crate::types::timestamp::Timestamp;
use crate::types::{
    channel::{Channel, ChannelMention},
    common::{Emoji, Resolved},
    poll::Poll,
    role::RoleSubscriptionData,
    sticker::{Sticker, StickerItem},
    user::User,
    Snowflake,
};

#[derive(Deserialize_repr, Serialize_repr, Debug, Eq, PartialEq, Clone)]
#[repr(u8)]
pub enum AttachmentFlags {
    IsRemix = 1 << 2,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Attachment {
    pub id: Snowflake,

    pub filename: String,

    pub description: Option<String>,
    pub content_type: Option<String>, // MIME type

    // Size in bytes
    pub size: usize,
    pub url: String,
    pub proxy_url: String,

    // Image
    pub width: Option<usize>,
    pub height: Option<usize>,

    // Voice message
    pub duration_secs: Option<f64>,
    pub waveform: Option<String>, // base64 encoded bytearray

    pub ephemeral: Option<bool>,
    pub flags: Option<u8>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ReactionCountDetails {
    pub burst: usize,
    pub normal: usize,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Reaction {
    pub count: usize,
    pub count_details: ReactionCountDetails,
    pub me: bool,
    pub me_burst: bool,
    pub emoji: Emoji,
    pub burst_colors: Vec<u32>,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Eq, PartialEq, Clone)]
#[repr(u64)]
pub enum MessageType {
    Default = 0,

    RecipientAdd = 1,
    RecipientRemove = 2,

    Call = 3,

    NameChange = 4,
    IconChange = 5,
    PinnedMessage = 6,

    UserJoin = 7,

    GuildBoost = 8,
    GuildBoostTier1 = 9,
    GuildBoostTier2 = 10,
    GuildBoostTier3 = 11,

    FollowAdd = 12,

    GuildDiscoveryDisqualified = 14,
    GuildDiscoveryRequalified = 15,
    GuildDiscoveryGracePeriodInitialWarning = 16,
    GuildDiscoveryGracePeriodFinalWarning = 17,

    ThreadCreated = 18,
    ThreadStarterMessage = 21,

    Reply = 19,
    ChatInputCommand = 20,

    GuildInviteReminder = 22,

    ContextMenuCommand = 23,
    AutoModerationAction = 24,
    RoleSubscriptionPurchase = 25,
    InteractionPremiumUpsell = 26,

    StageStart = 27,
    StageEnd = 28,
    StageSpeaker = 29,
    StageTopic = 31,

    GuildApplicationPremiumSubscription = 32,
    PurchaseNotification = 44,

    GuildIncidentAlertModeEnabled = 36,
    GuildIncidentAlertModeDisabled = 37,
    GuildIncidentReportRaid = 38,
    GuildIncidentReportFalseAlarm = 39,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Eq, PartialEq, Clone)]
#[repr(u8)]
pub enum ActivityType {
    Join = 1,
    Spectate = 2,
    Listen = 3,
    JoinRequest = 5,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Activity {
    pub message_activity_type: ActivityType,
    pub party_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Reference {
    pub message_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub guild_id: Option<Snowflake>,
    pub fail_if_not_exists: Option<bool>,
}

#[derive(Deserialize_repr, Serialize_repr, Debug)]
#[repr(u16)]
pub enum MessageFlag {
    Crossposted = 1 << 0,
    IsCrosspost = 1 << 1,
    SuppressEmbeds = 1 << 2,
    SourceMessageDeleted = 1 << 3,
    Urgent = 1 << 4,
    HasThread = 1 << 5,
    Ephemeral = 1 << 6,
    Loading = 1 << 7,
    FailedToMentionSomeRolesInThread = 1 << 8,
    SuppressNotifications = 1 << 12,
    IsVoiceMessage = 1 << 13,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Eq, PartialEq, Clone)]
#[repr(u8)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
    ApplicationCommandAutocomplete = 4,
    ModalSubmit = 5,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InteractionMetadata {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub interaction_type: InteractionType,
    pub user: User,

    pub original_response_message_id: Option<Snowflake>,
    pub interacted_message_id: Option<Snowflake>,
    pub triggering_interaction_metadata: Option<Box<InteractionMetadata>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Call {
    pub participants: Vec<Snowflake>,
    pub ended_timestamp: Option<Timestamp>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Message {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,

    pub content: String,

    pub author: Option<User>,

    pub timestamp: Timestamp,
    pub edited_timestamp: Option<Timestamp>,

    pub tts: bool,

    pub mentions: Vec<User>,
    pub mention_roles: Vec<Snowflake>,
    pub mention_channels: Option<ChannelMention>,
    pub mention_everyone: bool,

    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    pub reactions: Option<Vec<Reaction>>,
    pub nonce: Option<String>, // String "or integer"

    pub pinned: bool,

    pub webhook_id: Option<Snowflake>,

    #[serde(rename = "type")]
    pub message_type: MessageType,

    pub activity: Option<Activity>,

    #[serde(skip)]
    // TODO: Application type
    pub application: Option<()>,
    pub interaction_metadata: Option<InteractionMetadata>,

    pub application_id: Option<Snowflake>,
    pub message_reference: Option<Reference>,
    pub referenced_message: Option<Box<Message>>,

    pub thread: Option<Channel>,

    pub components: Option<Vec<component::Component>>,

    pub sticker_items: Option<Vec<StickerItem>>,
    pub stickers: Option<Vec<Sticker>>,

    pub position: Option<u64>,
    pub role_subscription_data: Option<RoleSubscriptionData>,

    pub resolved: Option<Resolved>,
    pub poll: Option<Poll>,

    pub flags: Option<u16>,
}

pub mod embed {
    use crate::types::timestamp::Timestamp;
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Footer {
        pub text: String, // 2048 chars

        #[serde(rename = "proxy_icon_url")]
        pub icon_proxy_url: Option<String>,
        pub icon_url: Option<String>,
    }

    impl Footer {
        pub fn new(text: &str) -> Self {
            Self {
                text: text.to_string(),
                icon_proxy_url: None,
                icon_url: None,
            }
        }

        pub fn icon(mut self, url: String) -> Self {
            self.icon_url = Some(url);
            self
        }

        pub fn icon_proxy(mut self, url: String) -> Self {
            self.icon_proxy_url = Some(url);
            self
        }
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Image {
        pub url: String,
        pub proxy_url: Option<String>,

        pub width: Option<usize>,
        pub height: Option<usize>,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Thumbnail {
        pub url: String,
        pub proxy_url: Option<String>,

        pub width: Option<usize>,
        pub height: Option<usize>,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Video {
        pub url: Option<String>,
        pub proxy_url: Option<String>,

        pub width: Option<usize>,
        pub height: Option<usize>,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Provider {
        pub name: Option<String>,
        pub url: Option<String>,
    }

    impl Provider {
        pub fn new(name: &str, url: String) -> Self {
            Self {
                name: Some(name.to_string()),
                url: Some(url),
            }
        }

        pub fn url(mut self, url: String) -> Self {
            self.url = Some(url);
            self
        }
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Author {
        pub name: String, // 256 chars
        pub url: Option<String>,

        #[serde(rename = "proxy_icon_url")]
        pub icon_proxy_url: Option<String>,
        pub icon_url: Option<String>,
    }

    impl Author {
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                url: None,
                icon_proxy_url: None,
                icon_url: None,
            }
        }

        pub fn url(mut self, url: String) -> Self {
            self.url = Some(url);
            self
        }

        pub fn icon_proxy_url(mut self, icon_proxy_url: String) -> Self {
            self.icon_proxy_url = Some(icon_proxy_url);
            self
        }

        pub fn icon_url(mut self, icon_url: String) -> Self {
            self.icon_url = Some(icon_url);
            self
        }
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Field {
        pub name: String,  // 256 chars
        pub value: String, // 1024 chars
        pub inline: Option<bool>,
    }

    impl Field {
        pub fn new(name: &str, value: &str) -> Self {
            Self {
                name: name.to_string(),
                value: value.to_string(),
                inline: None,
            }
        }

        pub fn inline(mut self, inline: bool) -> Self {
            self.inline = Some(inline);
            self
        }
    }

    // Max length of all text cannot exceed 6000 chars
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Embed {
        pub title: Option<String>, // 256 chars

        pub description: Option<String>, // 4096 chars
        pub url: Option<String>,

        pub timestamp: Option<Timestamp>,
        pub color: Option<u32>,

        pub footer: Option<Footer>,
        pub image: Option<Image>,
        pub thumbnail: Option<Thumbnail>,
        pub video: Option<Video>,
        pub provider: Option<Provider>,
        pub author: Option<Author>,

        pub fields: Option<Vec<Field>>, // 25 max
    }
}

pub mod component {
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use serde_repr::{Deserialize_repr, Serialize_repr};

    use crate::prelude::Snowflake;
    use crate::types::channel::ChannelType;
    use crate::types::common::Emoji;

    #[derive(Deserialize_repr, Serialize_repr, Debug, Eq, PartialEq, Clone)]
    #[repr(u8)]
    pub enum ComponentType {
        ActionRow = 1,
        Button = 2,
        StringSelect = 3,
        TextInput = 4,
        UserSelect = 5,
        RoleSelect = 6,
        MentionableSelect = 7,
        ChannelSelect = 8,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct ActionRow {
        #[serde(rename = "type")]
        _type: ComponentType,
        pub components: Option<Vec<Value>>,
    }

    impl ActionRow {
        pub fn new() -> Self {
            Self {
                _type: ComponentType::ActionRow,
                components: None,
            }
        }

        pub fn add_component(mut self, component: Component) -> Self {
            if self.components.is_none() {
                self.components = Some(Vec::new());
            }

            self.components
                .as_mut()
                .unwrap()
                .push(json!(component.build()));
            self
        }
    }

    #[derive(Deserialize_repr, Serialize_repr, Debug, Eq, PartialEq, Clone)]
    #[repr(u8)]
    pub enum ButtonStyle {
        Primary = 1,
        Secondary = 2,
        Success = 3,
        Danger = 4,
        Link = 5,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Button {
        #[serde(rename = "type")]
        _type: ComponentType,
        pub style: ButtonStyle,
        pub label: Option<String>,
        pub emoji: Option<Emoji>,
        pub custom_id: Option<String>,
        pub url: Option<String>,
        pub disabled: Option<bool>,
    }

    impl Button {
        pub fn new(id: &str, style: ButtonStyle) -> Self {
            Self {
                _type: ComponentType::Button,
                custom_id: Some(id.to_string()),
                style,
                label: None,
                emoji: None,
                url: None,
                disabled: None,
            }
        }

        pub fn label(mut self, label: String) -> Self {
            self.label = Some(label);
            self
        }

        pub fn emoji(mut self, emoji: Emoji) -> Self {
            self.emoji = Some(emoji);
            self
        }

        pub fn url(mut self, url: String) -> Self {
            self.url = Some(url);
            self
        }

        pub fn disabled(mut self, disabled: bool) -> Self {
            self.disabled = Some(disabled);
            self
        }
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct SelectOption {
        pub label: String,
        pub value: String,
        pub description: Option<String>,
        pub emoji: Option<Emoji>,
        pub default: Option<bool>,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[serde(rename_all = "lowercase")]
    pub enum DefaultValueType {
        User,
        Role,
        Channel,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct DefaultValue {
        pub id: Snowflake,
        #[serde(rename = "type")]
        pub value_type: DefaultValueType,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct SelectMenu {
        #[serde(rename = "type")]
        _type: ComponentType,
        pub custom_id: String,
        pub options: Option<Vec<SelectOption>>, // max 25, type 3
        pub channel_types: Option<Vec<ChannelType>>, // type 8
        pub placeholder: Option<String>,
        pub default_values: Option<Vec<DefaultValue>>, // type 5..=8
        pub min_values: Option<u8>,                    // 0..=25
        pub max_values: Option<u8>,                    // max 25
        pub disabled: Option<bool>,
    }

    impl SelectMenu {
        pub fn text(id: &str, options: Vec<SelectOption>) -> Self {
            Self {
                _type: ComponentType::StringSelect,
                options: Some(options),
                custom_id: id.to_string(),
                placeholder: None,
                channel_types: None,
                default_values: None,
                min_values: None,
                max_values: None,
                disabled: None,
            }
        }

        pub fn user(id: &str, options: Vec<SelectOption>) -> Self {
            Self {
                _type: ComponentType::UserSelect,
                options: Some(options),
                placeholder: None,
                custom_id: id.to_string(),
                channel_types: None,
                default_values: None,
                min_values: None,
                max_values: None,
                disabled: None,
            }
        }

        pub fn role(id: &str, options: Vec<SelectOption>) -> Self {
            Self {
                _type: ComponentType::RoleSelect,
                options: Some(options),
                placeholder: None,
                custom_id: id.to_string(),
                channel_types: None,
                default_values: None,
                min_values: None,
                max_values: None,
                disabled: None,
            }
        }

        pub fn mentionable(id: &str, options: Vec<SelectOption>) -> Self {
            Self {
                _type: ComponentType::MentionableSelect,
                options: Some(options),
                placeholder: None,
                custom_id: id.to_string(),
                channel_types: None,
                default_values: None,
                min_values: None,
                max_values: None,
                disabled: None,
            }
        }

        pub fn channel(id: &str, options: Vec<SelectOption>) -> Self {
            Self {
                _type: ComponentType::ChannelSelect,
                options: Some(options),
                placeholder: None,
                custom_id: id.to_string(),
                channel_types: None,
                default_values: None,
                min_values: None,
                max_values: None,
                disabled: None,
            }
        }

        pub fn channel_types(mut self, channel_types: Vec<ChannelType>) -> Self {
            if self._type != ComponentType::ChannelSelect {
                panic!("Channel types can only be set for ChannelSelect components");
            }

            self.channel_types = Some(channel_types);
            self
        }

        pub fn default_values(mut self, default_values: Vec<DefaultValue>) -> Self {
            self.default_values = Some(default_values);
            self
        }

        pub fn placeholder(mut self, placeholder: &str) -> Self {
            if self._type != ComponentType::StringSelect {
                panic!("Placeholder can only be set for StringSelect components");
            }

            self.placeholder = Some(placeholder.to_string());
            self
        }

        pub fn limits(mut self, min: u8, max: u8) -> Self {
            self.min_values = Some(min);
            self.max_values = Some(max);
            self
        }

        pub fn disabled(mut self, disabled: bool) -> Self {
            self.disabled = Some(disabled);
            self
        }
    }

    #[derive(Deserialize_repr, Serialize_repr, Debug, Eq, PartialEq, Clone)]
    #[repr(u8)]
    pub enum TextInputStyle {
        Short = 1,
        Paragraph = 2,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct TextInput {
        #[serde(rename = "type")]
        _type: ComponentType,
        pub custom_id: String,
        pub style: TextInputStyle,

        pub label: String, // max 25
        pub value: Option<String>,
        pub placeholder: Option<String>, // max 100

        pub min_length: Option<u16>, // 0..=4000
        pub max_length: Option<u16>, // 1..=4000

        pub required: Option<bool>,
    }

    impl TextInput {
        pub fn new(custom_id: &str, label: &str) -> Self {
            Self {
                _type: ComponentType::TextInput,
                custom_id: custom_id.to_string(),
                style: TextInputStyle::Short,
                label: label.to_string(),
                value: None,
                placeholder: None,
                min_length: None,
                max_length: None,
                required: None,
            }
        }

        pub fn style(mut self, style: TextInputStyle) -> Self {
            self.style = style;
            self
        }

        pub fn placeholder(mut self, placeholder: &str) -> Self {
            self.placeholder = Some(placeholder.to_string());
            self
        }

        pub fn value(mut self, value: &str) -> Self {
            self.value = Some(value.to_string());
            self
        }

        pub fn limits(mut self, min: u16, max: u16) -> Self {
            self.min_length = Some(min);
            self.max_length = Some(max);
            self
        }

        pub fn required(mut self, required: bool) -> Self {
            self.required = Some(required);
            self
        }
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[serde(untagged)]
    pub enum Component {
        ActionRow(ActionRow),
        Button(Box<Button>),
        StringSelect(SelectMenu),
        TextInput(TextInput),
        UserSelect(SelectMenu),
        RoleSelect(SelectMenu),
        MentionableSelect(SelectMenu),
        ChannelSelect(SelectMenu),
    }

    impl Component {
        pub fn build(self) -> Value {
            match self {
                Component::ActionRow(action_row) => json!(action_row),
                Component::Button(button) => json!(button),
                Component::StringSelect(select_menu) => json!(select_menu),
                Component::TextInput(text_input) => json!(text_input),
                Component::UserSelect(select_menu) => json!(select_menu),
                Component::RoleSelect(select_menu) => json!(select_menu),
                Component::MentionableSelect(select_menu) => json!(select_menu),
                Component::ChannelSelect(select_menu) => json!(select_menu),
            }
        }
    }
}

pub struct EmbedBuilder {
    value: Value,
}

impl EmbedBuilder {
    pub fn new() -> Self {
        Self { value: json!({}) }
    }

    pub fn author(mut self, author: embed::Author) -> Self {
        if author.name.len() > 256 {
            panic!("Author name must be less than 256 characters");
        }

        self.value["author"] = json!(author);
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        if title.len() > 256 {
            panic!("Title must be less than 256 characters");
        }

        self.value["title"] = json!(title);
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        if description.len() > 4096 {
            panic!("Description must be less than 4096 characters");
        }

        self.value["description"] = json!(description);
        self
    }

    pub fn url(mut self, url: String) -> Self {
        self.value["url"] = json!(url);
        self
    }

    pub fn color(mut self, color: u32) -> Self {
        self.value["color"] = json!(color);
        self
    }

    pub fn timestamp(mut self, timestamp: Timestamp) -> Self {
        self.value["timestamp"] = json!(timestamp);
        self
    }

    pub fn footer(mut self, footer: embed::Footer) -> Self {
        if footer.text.len() > 2048 {
            panic!("Footer text must be less than 2048 characters");
        }

        self.value["footer"] = json!(footer);
        self
    }

    pub fn image(mut self, image: embed::Image) -> Self {
        self.value["image"] = json!(image);
        self
    }

    pub fn thumbnail(mut self, thumbnail: embed::Thumbnail) -> Self {
        self.value["thumbnail"] = json!(thumbnail);
        self
    }

    pub fn video(mut self, video: embed::Video) -> Self {
        self.value["video"] = json!(video);
        self
    }

    pub fn provider(mut self, provider: embed::Provider) -> Self {
        self.value["provider"] = json!(provider);
        self
    }

    pub fn add_field(mut self, field: embed::Field) -> Self {
        if field.name.len() > 256 {
            panic!("Field name must be less than 256 characters");
        }

        if field.value.len() > 1024 {
            panic!("Field value must be less than 1024 characters");
        }

        let fields = match self.value["fields"].as_array_mut() {
            None => &mut Vec::new(),
            Some(v) => v,
        };

        if fields.len() == 25 {
            panic!("Can't have more than 25 fields in one embed");
        }

        fields.push(json!(field));
        self.value["fields"] = json!(fields);
        self
    }

    pub fn build(self) -> Value {
        self.value
    }
}

pub struct MessageBuilder {
    value: Value,
}

impl MessageBuilder {
    pub fn new(content: String) -> Self {
        Self {
            value: json!({
                "content": content,
            }),
        }
    }

    pub fn add_component(mut self, component: component::Component) -> Self {
        let components = match self.value["components"].as_array_mut() {
            None => &mut Vec::new(),
            Some(v) => v,
        };

        components.push(json!(component));
        self.value["components"] = json!(components);
        self
    }

    pub fn add_embed_from(mut self, embed: EmbedBuilder) -> Self {
        let embeds = match self.value["embeds"].as_array_mut() {
            None => &mut Vec::new(),
            Some(v) => v,
        };

        if embeds.len() == 10 {
            panic!("Can't have more than 10 embeds in one message");
        }

        embeds.push(embed.build());
        self.value["embeds"] = json!(embeds);
        self
    }

    pub fn add_embed(mut self, embed: Embed) -> Self {
        let embeds = match self.value["embeds"].as_array_mut() {
            None => &mut Vec::new(),
            Some(v) => v,
        };

        if embeds.len() == 10 {
            panic!("Can't have more than 10 embeds in one message");
        }

        embeds.push(json!(embed));
        self.value["embeds"] = json!(embeds);
        self
    }

    pub fn add_attachment(mut self, attachment: Attachment) -> Self {
        let attachments = match self.value["attachments"].as_array_mut() {
            None => &mut Vec::new(),
            Some(v) => v,
        };

        attachments.push(json!(attachment));
        self.value["attachments"] = json!(attachments);
        self
    }

    pub fn reference(mut self, message_reference: Reference) -> Self {
        self.value["message_reference"] = json!(message_reference);
        self
    }

    pub fn reference_to(mut self, message: &Message, fail: bool) -> Self {
        self.value["message_reference"] = json!({
            "message_id": message.id,
            "channel_id": message.channel_id,
            "guild_id": message.guild_id,
            "fail_if_not_exists": fail
        });
        self
    }

    pub fn poll(mut self, poll: Poll) -> Self {
        self.value["poll"] = json!(poll);
        self
    }

    pub fn build(self) -> Value {
        self.value
    }
}
