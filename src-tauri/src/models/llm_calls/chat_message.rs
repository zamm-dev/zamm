use crate::commands::Error;
use async_openai::types::{
    ChatCompletionRequestAssistantMessage, ChatCompletionRequestMessage,
    ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage,
    ChatCompletionRequestUserMessageContent, ChatCompletionResponseMessage, Role,
};
use diesel::backend::Backend;
use diesel::deserialize::FromSqlRow;
use diesel::deserialize::{self, FromSql};
use diesel::expression::AsExpression;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use ollama_rs::generation::chat::{
    ChatMessage as OllamaChatMessage, MessageRole as OllamaMessageRole,
};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    AsExpression,
    FromSqlRow,
    specta::Type,
)]
#[diesel(sql_type = Text)]
#[serde(tag = "role")]
pub enum ChatMessage {
    System { text: String },
    Human { text: String },
    AI { text: String },
}

impl TryFrom<ChatCompletionRequestMessage> for ChatMessage {
    type Error = Error;

    fn try_from(message: ChatCompletionRequestMessage) -> Result<Self, Self::Error> {
        match message {
            ChatCompletionRequestMessage::System(system_message) => {
                Ok(ChatMessage::System {
                    text: system_message.content,
                })
            }
            ChatCompletionRequestMessage::User(user_message) => {
                match user_message.content {
                    ChatCompletionRequestUserMessageContent::Text(text) => {
                        Ok(ChatMessage::Human { text })
                    }
                    ChatCompletionRequestUserMessageContent::Array(_) => {
                        Err(Error::UnexpectedOpenAiResponse {
                            reason: "Image chat not supported yet".to_string(),
                        })
                    }
                }
            }
            ChatCompletionRequestMessage::Assistant(assistant_message) => {
                match assistant_message.content {
                    Some(content) => Ok(ChatMessage::AI { text: content }),
                    None => Err(Error::UnexpectedOpenAiResponse {
                        reason: "AI function calls not supported yet".to_string(),
                    }),
                }
            }
            _ => Err(Error::UnexpectedOpenAiResponse {
                reason: "Only AI text chat is supported".to_string(),
            }),
        }
    }
}

impl TryFrom<ChatCompletionResponseMessage> for ChatMessage {
    type Error = Error;

    fn try_from(message: ChatCompletionResponseMessage) -> Result<Self, Self::Error> {
        let text = message.content.ok_or(Error::UnexpectedOpenAiResponse {
            reason: "No content in response".to_string(),
        })?;
        match message.role {
            Role::System => Ok(ChatMessage::System { text }),
            Role::User => Ok(ChatMessage::Human { text }),
            Role::Assistant => Ok(ChatMessage::AI { text }),
            _ => Err(Error::UnexpectedOpenAiResponse {
                reason: "Only AI text chat is supported".to_string(),
            }),
        }
    }
}

impl From<OllamaChatMessage> for ChatMessage {
    fn from(message: OllamaChatMessage) -> Self {
        let text = message.content;
        match message.role {
            OllamaMessageRole::System => ChatMessage::System { text },
            OllamaMessageRole::User => ChatMessage::Human { text },
            OllamaMessageRole::Assistant => ChatMessage::AI { text },
        }
    }
}

impl From<ChatMessage> for ChatCompletionRequestMessage {
    fn from(val: ChatMessage) -> Self {
        match val {
            ChatMessage::System { text } => ChatCompletionRequestMessage::System(
                ChatCompletionRequestSystemMessage {
                    content: text,
                    role: Role::System,
                    ..Default::default()
                },
            ),
            ChatMessage::Human { text } => {
                ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
                    content: ChatCompletionRequestUserMessageContent::Text(text),
                    role: Role::User,
                    ..Default::default()
                })
            }
            ChatMessage::AI { text } => ChatCompletionRequestMessage::Assistant(
                ChatCompletionRequestAssistantMessage {
                    content: Some(text),
                    role: Role::Assistant,
                    ..Default::default()
                },
            ),
        }
    }
}

impl From<ChatMessage> for OllamaChatMessage {
    fn from(val: ChatMessage) -> Self {
        match val {
            ChatMessage::System { text } => OllamaChatMessage::system(text),
            ChatMessage::Human { text } => OllamaChatMessage::user(text),
            ChatMessage::AI { text } => OllamaChatMessage::assistant(text),
        }
    }
}

impl ToSql<Text, Sqlite> for ChatMessage
where
    String: ToSql<Text, Sqlite>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let json_str = serde_json::to_string(&self)?;
        out.set_value(json_str);
        Ok(IsNull::No)
    }
}

impl<DB> FromSql<Text, DB> for ChatMessage
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let json_str = String::from_sql(bytes)?;
        let parsed_json: Self = serde_json::from_str(&json_str)?;
        Ok(parsed_json)
    }
}
