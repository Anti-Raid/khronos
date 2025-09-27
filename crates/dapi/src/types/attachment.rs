use serde::{ser::SerializeSeq, Deserialize, Serialize};
use serenity::all::*;

const ALLOW_NEW_ATTACHMENTS: bool = false;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SingleCreateMessageAttachment {
    pub filename: String,
    pub description: Option<String>,
    pub content: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExistingAttachment {
    id: AttachmentId,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)] // Serde needs to do either id only for existing or filename/description/content for new
pub enum NewOrExisting {
    New(SingleCreateMessageAttachment),
    Existing(ExistingAttachment),
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CreateMessageAttachment {
    #[serde(flatten)]
    pub new_and_existing_attachments: Vec<NewOrExisting>,
}

impl Serialize for CreateMessageAttachment {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct NewAttachment<'a> {
            id: u64,
            filename: &'a str,
            description: &'a Option<String>,
        }

        // Instead of an `AttachmentId`, the `id` field for new attachments corresponds to the
        // index of the new attachment in the multipart payload. The attachment data will be
        // labeled with `files[{id}]` in the multipart body. See `Multipart::build_form`.
        let mut id = 0;
        let mut seq = serializer.serialize_seq(Some(self.new_and_existing_attachments.len()))?;
        for attachment in &self.new_and_existing_attachments {
            match attachment {
                NewOrExisting::New(new_attachment) => {
                    let attachment = NewAttachment {
                        id,
                        filename: &new_attachment.filename,
                        description: &new_attachment.description,
                    };
                    id += 1;
                    seq.serialize_element(&attachment)?;
                }
                NewOrExisting::Existing(existing_attachment) => {
                    seq.serialize_element(existing_attachment)?;
                }
            }
        }
        seq.end()
    }
}

impl CreateMessageAttachment {
    pub fn take_files<'a>(&self) -> Result<Vec<serenity::all::CreateAttachment<'a>>, crate::Error> {
        pub const MESSAGE_ATTACHMENT_DESCRIPTION_LIMIT: usize = 1024;
        pub const MESSAGE_ATTACHMENT_CONTENT_BYTES_LIMIT: usize = 8 * 1024 * 1024; // 8 MB
        pub const MESSAGE_MAX_ATTACHMENT_COUNT: usize = 3;

        if self.new_and_existing_attachments.len() > MESSAGE_MAX_ATTACHMENT_COUNT {
            return Err(
                format!("Too many attachments, limit is {MESSAGE_MAX_ATTACHMENT_COUNT}",).into(),
            );
        }

        let mut attachments = Vec::new();
        for attachment in &self.new_and_existing_attachments {
            if let NewOrExisting::New(new_attachment) = attachment {
                if !ALLOW_NEW_ATTACHMENTS {
                    return Err("Message attachments are disabled right now due to ongoing maintenance and security improvements".into());
                }

                let desc = new_attachment.description.clone();
                let desc = desc.unwrap_or_default();

                if desc.len() > MESSAGE_ATTACHMENT_DESCRIPTION_LIMIT {
                    return Err(format!(
                        "Attachment description exceeds limit of {MESSAGE_ATTACHMENT_DESCRIPTION_LIMIT}",
                    )
                    .into());
                }

                let content = &new_attachment.content;

                if content.is_empty() {
                    return Err("Attachment content cannot be empty".into());
                }

                if content.len() > MESSAGE_ATTACHMENT_CONTENT_BYTES_LIMIT {
                    return Err(format!(
                        "Attachment content exceeds limit of {MESSAGE_ATTACHMENT_CONTENT_BYTES_LIMIT} bytes",
                    )
                    .into());
                }

                let mut ca = serenity::all::CreateAttachment::bytes(
                    content.clone(),
                    new_attachment.filename.clone(),
                );

                if !desc.is_empty() {
                    ca = ca.description(desc.clone());
                }

                attachments.push(ca);
            }
        }

        Ok(attachments)
    }
}
