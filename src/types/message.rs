use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::{
    channel::{Channel, ChannelMention},
    common::{Emoji, Resolved},
    poll::Poll,
    role::RoleSubscriptionData,
    Snowflake,
    sticker::{Sticker, StickerItem},
    user::User
};

#[derive(Deserialize_repr, Serialize_repr, Debug)]
#[repr(u8)]
pub enum AttachmentFlags {
    IsRemix = 1 << 2,
}

#[derive(Serialize, Deserialize, Debug)]
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

    pub ephemeral: bool,
    pub flags: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReactionCountDetails {
    pub burst: usize,
    pub normal: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reaction {
    pub count: usize,
    pub count_details: ReactionCountDetails,
    pub me: bool,
    pub me_burst: bool,
    pub emoji: Emoji,
    pub burst_colors: Vec<u32>,
}

#[derive(Deserialize_repr, Serialize_repr, Debug)]
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

#[derive(Deserialize_repr, Serialize_repr, Debug)]
#[repr(u8)]
pub enum ActivityType {
    Join = 1,
    Spectate = 2,
    Listen = 3,
    JoinRequest = 5,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Activity {
    pub message_activity_type: ActivityType,
    pub party_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Deserialize_repr, Serialize_repr, Debug)]
#[repr(u8)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
    ApplicationCommandAutocomplete = 4,
    ModalSubmit = 5,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InteractionMetadata {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub interaction_type: InteractionType,
    pub user: User,

    pub original_response_message_id: Option<Snowflake>,
    pub interacted_message_id: Option<Snowflake>,
    pub triggering_interaction_metadata: Option<Box<InteractionMetadata>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Call {
    pub participants: Vec<Snowflake>,
    pub ended_timestamp: Option<String>, // ISO8601
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: Snowflake,
    pub channel_id: Snowflake,

    pub content: String,

    pub author: Option<User>,

    pub timestamp: String, // ISO8601
    pub edited_timestamp: Option<String>,

    pub tts: bool,

    pub mentions: Vec<User>,
    pub mention_roles: Vec<Snowflake>,
    pub mention_channels: Option<ChannelMention>,
    pub mention_everyone: bool,

    pub attachments: Vec<Attachment>,
    pub embeds: Vec<embed::Embed>,
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
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Footer {
        pub text: String, // 2048 chars

        #[serde(rename = "proxy_icon_url")]
        pub icon_proxy_url: Option<String>,
        pub icon_url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Image {
        pub url: String,
        pub proxy_url: Option<String>,

        pub width: Option<usize>,
        pub height: Option<usize>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Thumbnail {
        pub url: String,
        pub proxy_url: Option<String>,

        pub width: Option<usize>,
        pub height: Option<usize>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Video {
        pub url: Option<String>,
        pub proxy_url: Option<String>,

        pub width: Option<usize>,
        pub height: Option<usize>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Provider {
        pub name: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Author {
        pub name: String, // 256 chars
        pub url: Option<String>,

        #[serde(rename = "proxy_icon_url")]
        pub icon_proxy_url: Option<String>,
        pub icon_url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Field {
        pub name: String, // 256 chars
        pub value: String, // 1024 chars
        pub inline: Option<bool>,
    }

    // Max length of all text cannot exceed 6000 chars
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Embed {
        pub title: Option<String>, // 256 chars

        pub description: Option<String>, // 4096 chars
        pub url: Option<String>,

        pub timestamp: Option<String>, // ISO8601
        pub color: Option<u32>,

        pub footer: Option<Footer>,
        pub image: Option<Image>,
        pub thumbnail: Option<Thumbnail>,
        pub video: Option<Video>,
        pub provider: Option<Provider>,
        pub author: Option<Author>,

        pub fields: Option<Vec<Field>> // 25 max
    }
}

pub mod component {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use serde_repr::{Deserialize_repr, Serialize_repr};

    use crate::prelude::Snowflake;
    use crate::types::channel::ChannelType;
    use crate::types::common::Emoji;

    #[derive(Deserialize_repr, Serialize_repr, Debug)]
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

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ActionRow {
        #[serde(rename = "type")]
        _type: ComponentType,
        pub components: Option<Vec<Value>>,
    }

    #[derive(Deserialize_repr, Serialize_repr, Debug)]
    #[repr(u8)]
    pub enum ButtonStyle {
        Primary = 1,
        Secondary = 2,
        Success = 3,
        Danger = 4,
        Link = 5,
    }

    #[derive(Serialize, Deserialize, Debug)]
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

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SelectOption {
        pub label: String,
        pub value: String,
        pub description: Option<String>,
        pub emoji: Option<Emoji>,
        pub default: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "lowercase")]
    pub enum DefaultValueType {
        User,
        Role,
        Channel,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct DefaultValue {
        pub id: Snowflake,
        #[serde(rename = "type")]
        pub value_type: DefaultValueType,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SelectMenu {
        #[serde(rename = "type")]
        _type: ComponentType,
        pub custom_id: Option<String>,
        pub options: Option<Vec<SelectOption>>, // max 25, type 3
        pub channel_types: Option<Vec<ChannelType>>, // type 8
        pub placeholder: Option<String>,
        pub default_values: Option<Vec<DefaultValue>>, // type 5..=8
        pub min_values: Option<u8>, // 0..=25
        pub max_values: Option<u8>, // max 25
        pub disabled: Option<bool>,
    }

    #[derive(Deserialize_repr, Serialize_repr, Debug)]
    #[repr(u8)]
    pub enum TextInputStyle {
        Short = 1,
        Paragraph = 2,
    }

    #[derive(Serialize, Deserialize, Debug)]
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

    #[derive(Serialize, Deserialize, Debug)]
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
}


pub struct EmbedBuilder {
    value: Value,
}

impl EmbedBuilder {
    pub fn new() -> Self {
        Self {
            value: json!({}),
        }
    }

    pub fn set_author(&mut self, author: embed::Author) {
        if author.name.len() > 256 {
            panic!("Author name must be less than 256 characters");
        }

        self.value["author"] = json!(author);
    }

    pub fn set_title(&mut self, title: String) {
        if title.len() > 256 {
            panic!("Title must be less than 256 characters");
        }

        self.value["title"] = json!(title);
    }

    pub fn set_description(&mut self, description: String) {
        if description.len() > 4096 {
            panic!("Description must be less than 4096 characters");
        }

        self.value["description"] = json!(description);
    }

    pub fn set_url(&mut self, url: String) {
        self.value["url"] = json!(url);
    }

    pub fn set_color(&mut self, color: u32) {
        self.value["color"] = json!(color);
    }

    pub fn set_timestamp(&mut self, timestamp: String) {
        self.value["timestamp"] = json!(timestamp);
    }

    pub fn set_footer(&mut self, footer: embed::Footer) {
        if footer.text.len() > 2048 {
            panic!("Footer text must be less than 2048 characters");
        }

        self.value["footer"] = json!(footer);
    }

    pub fn set_image(&mut self, image: embed::Image) {
        self.value["image"] = json!(image);
    }

    pub fn set_thumbnail(&mut self, thumbnail: embed::Thumbnail) {
        self.value["thumbnail"] = json!(thumbnail);
    }

    pub fn set_video(&mut self, video: embed::Video) {
        self.value["video"] = json!(video);
    }

    pub fn set_provider(&mut self, provider: embed::Provider) {
        self.value["provider"] = json!(provider);
    }

    pub fn add_field(&mut self, field: embed::Field) {
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
                "flags": 0,
            }),
        }
    }

    pub fn add_component(&mut self, component: component::Component) {
        let components = match self.value["components"].as_array_mut() {
            None => &mut Vec::new(),
            Some(v) => v,
        };

        components.push(json!(component));
        self.value["components"] = json!(components);
    }

    pub fn add_embed(&mut self, embed: embed::Embed) {
        let embeds = match self.value["embeds"].as_array_mut() {
            None => &mut Vec::new(),
            Some(v) => v,
        };

        if embeds.len() == 10 {
            panic!("Can't have more than 10 embeds in one message");
        }

        embeds.push(json!(embed));
        self.value["embeds"] = json!(embeds);
    }

    pub fn add_attachment(&mut self, attachment: Attachment) {
        let attachments = match self.value["attachments"].as_array_mut() {
            None => &mut Vec::new(),
            Some(v) => v,
        };

        attachments.push(json!(attachment));
        self.value["attachments"] = json!(attachments);
    }

    pub fn set_reference(&mut self, message_reference: Reference) {
        self.value["message_reference"] = json!(message_reference);
    }

    pub fn set_poll(&mut self, poll: Poll) {
        self.value["poll"] = json!(poll);
    }

    pub fn build(self) -> Value {
        self.value
    }
}



