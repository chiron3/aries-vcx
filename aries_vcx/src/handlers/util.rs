use messages::{
    msg_fields::protocols::{
        connection::{invitation::Invitation, Connection},
        cred_issuance::CredentialIssuance,
        discover_features::DiscoverFeatures,
        out_of_band::{invitation::Invitation as OobInvitation, OutOfBand},
        present_proof::{
            propose::{Predicate, PresentationAttr},
            PresentProof,
        },
        report_problem::ProblemReport,
        revocation::Revocation,
        trust_ping::TrustPing,
    },
    AriesMessage,
};
use strum_macros::{AsRefStr, EnumString};

use crate::errors::error::{AriesVcxError, AriesVcxErrorKind, VcxResult};

macro_rules! matches_thread_id {
    ($msg:expr, $id:expr) => {
        $msg.decorators.thread.thid == $id || $msg.decorators.thread.pthid.as_deref() == Some($id)
    };
}

macro_rules! matches_opt_thread_id {
    ($msg:expr, $id:expr) => {
        match $msg.decorators.thread.as_ref() {
            Some(t) => t.thid == $id || t.pthid.as_deref() == Some($id),
            None => true,
        }
    };
}

macro_rules! get_attach_as_string {
    ($attachments:expr) => {{
        let __attach = $attachments.get(0).as_ref().map(|a| &a.data.content);
        let Some(messages::decorators::attachment::AttachmentType::Base64(encoded_attach)) = __attach else {
                                    return Err(AriesVcxError::from_msg(
                                        AriesVcxErrorKind::SerializationError,
                                        format!("Attachment is not base 64 encoded JSON: {:?}", $attachments.get(0)),
                                    ));
                                };
        let Ok(bytes) = base64::decode(encoded_attach) else {
                                    return Err(AriesVcxError::from_msg(
                                        AriesVcxErrorKind::SerializationError,
                                        format!("Attachment is not base 64 encoded JSON: {:?}", $attachments.get(0)),
                                    ));
                                };
        let Ok(attach_string) = String::from_utf8(bytes) else {
                                    return Err(AriesVcxError::from_msg(
                                        AriesVcxErrorKind::SerializationError,
                                        format!("Attachment is not base 64 encoded JSON: {:?}", $attachments.get(0)),
                                    ));
                                };

        attach_string
    }};
}

macro_rules! make_attach_from_str {
    ($str_attach:expr, $id:expr) => {{
        let attach_type = messages::decorators::attachment::AttachmentType::Base64(base64::encode($str_attach));
        let attach_data = messages::decorators::attachment::AttachmentData::new(attach_type);
        let mut attach = messages::decorators::attachment::Attachment::new(attach_data);
        attach.id = Some($id);
        attach.mime_type = Some(messages::misc::MimeType::Json);
        attach
    }};
}

pub(crate) use get_attach_as_string;
pub(crate) use make_attach_from_str;
pub(crate) use matches_opt_thread_id;
pub(crate) use matches_thread_id;

pub fn verify_thread_id(thread_id: &str, message: &AriesMessage) -> VcxResult<()> {
    let is_match = match message {
        AriesMessage::BasicMessage(msg) => matches_opt_thread_id!(msg, thread_id),
        AriesMessage::Connection(Connection::Invitation(Invitation::Public(msg))) => msg.id == thread_id,
        AriesMessage::Connection(Connection::Invitation(Invitation::Pairwise(msg))) => msg.id == thread_id,
        AriesMessage::Connection(Connection::Invitation(Invitation::PairwiseDID(msg))) => msg.id == thread_id,
        AriesMessage::Connection(Connection::ProblemReport(msg)) => matches_thread_id!(msg, thread_id),
        AriesMessage::Connection(Connection::Request(msg)) => matches_opt_thread_id!(msg, thread_id),
        AriesMessage::Connection(Connection::Response(msg)) => matches_thread_id!(msg, thread_id),
        AriesMessage::CredentialIssuance(CredentialIssuance::Ack(msg)) => matches_thread_id!(msg, thread_id),
        AriesMessage::CredentialIssuance(CredentialIssuance::IssueCredential(msg)) => {
            matches_thread_id!(msg, thread_id)
        }
        AriesMessage::CredentialIssuance(CredentialIssuance::OfferCredential(msg)) => {
            matches_opt_thread_id!(msg, thread_id)
        }
        AriesMessage::CredentialIssuance(CredentialIssuance::ProposeCredential(msg)) => {
            matches_opt_thread_id!(msg, thread_id)
        }
        AriesMessage::CredentialIssuance(CredentialIssuance::RequestCredential(msg)) => {
            matches_opt_thread_id!(msg, thread_id)
        }
        AriesMessage::DiscoverFeatures(DiscoverFeatures::Query(msg)) => msg.id == thread_id,
        AriesMessage::DiscoverFeatures(DiscoverFeatures::Disclose(msg)) => matches_thread_id!(msg, thread_id),
        AriesMessage::Notification(msg) => matches_thread_id!(msg, thread_id),
        AriesMessage::OutOfBand(OutOfBand::Invitation(msg)) => msg.id == thread_id,
        AriesMessage::OutOfBand(OutOfBand::HandshakeReuse(msg)) => matches_thread_id!(msg, thread_id),
        AriesMessage::OutOfBand(OutOfBand::HandshakeReuseAccepted(msg)) => matches_thread_id!(msg, thread_id),
        AriesMessage::PresentProof(PresentProof::Ack(msg)) => matches_thread_id!(msg, thread_id),
        AriesMessage::PresentProof(PresentProof::Presentation(msg)) => matches_thread_id!(msg, thread_id),
        AriesMessage::PresentProof(PresentProof::ProposePresentation(msg)) => matches_opt_thread_id!(msg, thread_id),
        AriesMessage::PresentProof(PresentProof::RequestPresentation(msg)) => matches_opt_thread_id!(msg, thread_id),
        AriesMessage::ReportProblem(msg) => matches_opt_thread_id!(msg, thread_id),
        AriesMessage::Revocation(Revocation::Revoke(msg)) => matches_opt_thread_id!(msg, thread_id),
        AriesMessage::Revocation(Revocation::Ack(msg)) => matches_thread_id!(msg, thread_id),
        AriesMessage::Routing(msg) => msg.id == thread_id,
        AriesMessage::TrustPing(TrustPing::Ping(msg)) => matches_opt_thread_id!(msg, thread_id),
        AriesMessage::TrustPing(TrustPing::PingResponse(msg)) => matches_thread_id!(msg, thread_id),
    };

    if !is_match {
        return Err(AriesVcxError::from_msg(
            AriesVcxErrorKind::InvalidJson,
            format!(
                "Cannot handle message {:?}: thread id does not match, expected {:?}",
                message, thread_id
            ),
        ));
    };

    Ok(())
}

#[derive(Debug, Clone, AsRefStr, EnumString, PartialEq)]
pub enum AttachmentId {
    #[strum(serialize = "libindy-cred-offer-0")]
    CredentialOffer,
    #[strum(serialize = "libindy-cred-request-0")]
    CredentialRequest,
    #[strum(serialize = "libindy-cred-0")]
    Credential,
    #[strum(serialize = "libindy-request-presentation-0")]
    PresentationRequest,
    #[strum(serialize = "libindy-presentation-0")]
    Presentation,
}

/// For retro-fitting the new messages.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum AnyInvitation {
    Con(Invitation),
    Oob(OobInvitation),
}

// todo: this is shared by multiple protocols to express different things - needs to be split
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Status {
    Undefined,
    Success,
    Failed(ProblemReport),
    Declined(ProblemReport),
}

impl Status {
    pub fn code(&self) -> u32 {
        match self {
            Status::Undefined => 0,
            Status::Success => 1,
            Status::Failed(_) => 2,
            Status::Declined(_) => 3,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct CredentialData {
    pub schema_id: String,
    pub cred_def_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rev_reg_id: Option<String>,
    pub values: serde_json::Value,
    pub signature: serde_json::Value,
    pub signature_correctness_proof: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rev_reg: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OfferInfo {
    pub credential_json: String,
    pub cred_def_id: String,
    pub rev_reg_id: Option<String>,
    pub tails_file: Option<String>,
}

impl OfferInfo {
    pub fn new(
        credential_json: String,
        cred_def_id: String,
        rev_reg_id: Option<String>,
        tails_file: Option<String>,
    ) -> Self {
        Self {
            credential_json,
            cred_def_id,
            rev_reg_id,
            tails_file,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct PresentationProposalData {
    pub attributes: Vec<PresentationAttr>,
    pub predicates: Vec<Predicate>,
    pub comment: Option<String>,
}
