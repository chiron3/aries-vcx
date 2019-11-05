use v3::messages::MessageId;
use messages::thread::Thread;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PresentationProposal {
    #[serde(rename = "@id")]
    pub id: MessageId,
    pub comment: Option<String>,
    pub presentation_proposal: PresentationPreview,
    #[serde(rename = "~thread")]
    pub thread: Thread,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PresentationPreview {
    pub attributes: Vec<Attribute>,
    pub predicates: Vec<Predicate>
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub cred_def_id: Option<String>,
    #[serde(rename = "mime-type")]
    pub mime_type: Option<String>,
    // TODO: FIXME
    pub value: Option<String> // TODO: FIXME
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Predicate {
    pub name: String,
    pub cred_def_id: Option<String>,
    pub predicate: String,
    pub threshold: i64,
    pub filter: Vec<::serde_json::Value>
}

impl PresentationProposal {
    pub fn create() -> Self {
        PresentationProposal::default()
    }
}

impl Default for PresentationProposal {
    fn default() -> PresentationProposal {
        PresentationProposal {
            id: MessageId::new(),
            comment: None,
            presentation_proposal: PresentationPreview::default(),
            thread: Thread::new(),
        }
    }
}

impl Default for PresentationPreview {
    fn default() -> PresentationPreview {
        PresentationPreview {
            attributes: Vec::new(),
            predicates: Vec::new(),
        }
    }
}
