use v3::messages::{MessageId, MessageType, A2AMessage, A2AMessageKinds};
use v3::messages::attachment::{Attachments, Attachment, Json, AttachmentEncoding};
use error::{VcxError, VcxResult, VcxErrorKind};
use messages::thread::Thread;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CredentialRequest {
    #[serde(rename = "@id")]
    pub id: MessageId,
    pub comment: String,
    #[serde(rename = "requests~attach")]
    pub requests_attach: Attachments,
    pub thread: Thread
}

impl CredentialRequest {
    pub fn create() -> Self {
        CredentialRequest {
            id: MessageId::new(),
            comment: String::new(),
            requests_attach: Attachments::new(),
            thread: Thread::new(),
        }
    }

    pub fn set_comment(mut self, comment: String) -> Self {
        self.comment = comment;
        self
    }

    pub fn set_requests_attach(mut self, credential_request: String) -> VcxResult<CredentialRequest> {
        let json: Json = Json::new(::serde_json::Value::String(credential_request), AttachmentEncoding::Base64)?;
        self.requests_attach.add(Attachment::JSON(json));
        Ok(self)
    }

    pub fn set_thread(mut self, thread: Thread) -> Self {
        self.thread = thread;
        self
    }
}