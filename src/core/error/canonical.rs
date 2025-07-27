use crate::{
    app::constant::UNKNOWN,
    common::model::{ApiStatus, GenericError},
    core::{
        aiserver::v1::{CustomErrorDetails, ErrorDetails},
        model::{anthropic, openai},
    },
};

pub struct CanonicalError {
    pub code: Option<String>,
    pub details: Option<CustomErrorDetails>,
    pub status_code: u16,
    pub r#type: &'static str,
}

impl From<ErrorDetails> for CanonicalError {
    #[inline]
    fn from(error: ErrorDetails) -> Self {
        Self {
            code: None,
            details: error.details,
            status_code: ErrorDetails::status_code(error.error),
            r#type: ErrorDetails::r#type(error.error),
        }
    }
}

impl CanonicalError {
    #[inline]
    pub const fn unknown() -> Self {
        Self {
            code: None,
            details: None,
            status_code: 500,
            r#type: UNKNOWN,
        }
    }

    #[inline]
    pub fn with_code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }
}

impl ::core::iter::Sum for CanonicalError {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut code: Option<String> = None;
        let mut details: Option<CustomErrorDetails> = None;
        let mut status_code = 0;
        let mut r#type = UNKNOWN;

        for e in iter {
            if let Some(acode) = e.code {
                if let Some(code) = code.as_mut() {
                    code.push_str(&acode);
                } else {
                    code = Some(acode);
                }
            }
            if let Some(adetails) = e.details {
                if let Some(details) = details.as_mut() {
                    details.add(adetails);
                } else {
                    details = Some(adetails);
                }
            }
            if status_code < e.status_code {
                status_code = e.status_code;
                r#type = e.r#type;
            }
        }

        Self {
            code,
            details,
            status_code,
            r#type,
        }
    }
}

impl CanonicalError {
    #[inline]
    pub fn title(&self) -> Option<String> {
        match &self.details {
            Some(details) => Some(details.title.clone()),
            None => self.code.as_ref().map(|s| s.replace("_", " ")),
        }
    }

    #[inline]
    pub fn detail(&self) -> Option<&str> {
        self.details.as_ref().map(|details| details.detail.as_str())
    }

    #[inline]
    pub fn status_code(&self) -> ::http::StatusCode {
        unsafe {
            ::core::intrinsics::transmute_unchecked(crate::app::constant::status::StatusCode(
                self.status_code,
            ))
        }
    }

    #[inline]
    pub fn into_generic(self) -> GenericError {
        use std::borrow::Cow;

        let code = match self.code {
            Some(code) => Cow::Owned(code),
            None => Cow::Borrowed(UNKNOWN),
        };

        let message = if let Some(details) = self.details {
            #[derive(::serde::Serialize)]
            struct Message {
                code: Cow<'static, str>,
                details: CustomErrorDetails,
            }

            Cow::Owned(__unwrap!(serde_json::to_string(&Message { code, details })))
        } else {
            code
        };

        GenericError {
            status: ApiStatus::Error,
            code: Some(self.status_code),
            error: Some(Cow::Borrowed(self.r#type)),
            message: Some(message),
        }
    }

    #[inline]
    pub fn into_openai(self) -> openai::OpenAiError {
        use std::borrow::Cow;

        let message = if let Some(details) = self.details {
            Cow::Owned(__unwrap!(serde_json::to_string(&details)))
        } else {
            Cow::Borrowed(UNKNOWN)
        };

        openai::ErrorDetail {
            code: self.code.map(Cow::Owned),
            message,
        }
        .into_openai()
    }

    #[inline]
    pub fn into_anthropic(self) -> anthropic::AnthropicError {
        use std::borrow::Cow;

        let code = match self.code {
            Some(code) => Cow::Owned(code),
            None => Cow::Borrowed(UNKNOWN),
        };

        let message = if let Some(details) = self.details {
            #[derive(::serde::Serialize)]
            struct Message {
                code: Cow<'static, str>,
                details: CustomErrorDetails,
            }

            Cow::Owned(__unwrap!(serde_json::to_string(&Message { code, details })))
        } else {
            code
        };

        anthropic::ErrorDetail {
            r#type: self.r#type,
            message,
        }
        .into_anthropic()
    }
}
