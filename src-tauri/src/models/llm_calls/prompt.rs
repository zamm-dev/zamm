use crate::commands::Error;
use crate::models::llm_calls::chat_message::ChatMessage;
use async_openai::types::ChatCompletionRequestMessage;
use diesel::backend::Backend;
use diesel::deserialize::FromSqlRow;
use diesel::deserialize::{self, FromSql};
use diesel::expression::AsExpression;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};
use serde_json;
use std::ops::Deref;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, specta::Type)]
pub struct ChatPrompt {
    pub messages: Vec<ChatMessage>,
}

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
#[serde(tag = "type")]
pub enum Prompt {
    Chat(ChatPrompt),
}

impl Deref for ChatPrompt {
    type Target = Vec<ChatMessage>;

    fn deref(&self) -> &Self::Target {
        &self.messages
    }
}

impl TryFrom<Vec<ChatCompletionRequestMessage>> for ChatPrompt {
    type Error = Error;

    fn try_from(
        messages: Vec<ChatCompletionRequestMessage>,
    ) -> Result<Self, Self::Error> {
        let messages = messages
            .into_iter()
            .map(|message| message.try_into())
            .collect::<Result<Vec<ChatMessage>, Self::Error>>()?;
        Ok(ChatPrompt { messages })
    }
}

impl From<ChatPrompt> for Vec<ChatCompletionRequestMessage> {
    fn from(val: ChatPrompt) -> Self {
        val.messages
            .into_iter()
            .map(|message| message.into())
            .collect()
    }
}

impl ToSql<Text, Sqlite> for Prompt
where
    String: ToSql<Text, Sqlite>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let json_str = serde_json::to_string(&self)?;
        out.set_value(json_str);
        Ok(IsNull::No)
    }
}

impl<DB> FromSql<Text, DB> for Prompt
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
