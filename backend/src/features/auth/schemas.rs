use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email, length(max = 255))]
    pub email: String,
    #[validate(length(min = 4, max = 255))]
    pub username: String,
    #[validate(length(min = 8, max = 255))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub user_id: String,
    pub session_token: String,
}

#[derive(Debug, Deserialize, Validate)]
#[validate(schema(function = "validate_login_request"))]
pub struct LoginRequest {
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub password: String,
}

fn validate_login_request(login_request: &LoginRequest) -> Result<(), ValidationError> {
    if login_request.username.is_none() && login_request.email.is_none() {
        return Err(ValidationError::new("username_or_email_required")
            .with_message(Cow::from("Either username or email is required")));
    }

    if login_request.username.is_some() && login_request.email.is_some() {
        return Err(
            ValidationError::new("username_and_email_conflict").with_message(Cow::from(
                "Only one of username or email should be provided",
            )),
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_register_request_validation() {
        let valid_request = RegisterRequest {
            email: "abc@example.com".to_string(),
            username: "valid_username".to_string(),
            password: "valid_password".to_string(),
        };

        assert!(valid_request.validate().is_ok());
    }

    #[test]
    fn test_register_request_validation_invalid_email() {
        let invalid_request = RegisterRequest {
            email: "invalid_email".to_string(),
            username: "valid_username".to_string(),
            password: "valid_password".to_string(),
        };

        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_register_request_validation_short_username() {
        let invalid_request = RegisterRequest {
            email: "abc@example.com".to_string(),
            username: "a".to_string(),
            password: "valid_password".to_string(),
        };

        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_register_request_validation_short_password() {
        let invalid_request = RegisterRequest {
            email: "abc@example.com".to_string(),
            username: "valid_username".to_string(),
            password: "shortpw".to_string(),
        };
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_login_request_username_validation() {
        let valid_request = LoginRequest {
            username: Some("valid_username".to_string()),
            email: None,
            password: "valid_password".to_string(),
        };

        assert!(valid_request.validate().is_ok());
    }

    #[test]
    fn test_login_request_email_validation() {
        let valid_request = LoginRequest {
            username: None,
            email: Some("abc@example.com".to_string()),
            password: "valid_password".to_string(),
        };
        assert!(valid_request.validate().is_ok());
    }

    #[test]
    fn test_login_request_both_username_and_email_validation() {
        let invalid_request = LoginRequest {
            username: Some("valid_username".to_string()),
            email: Some("abc@example.com".to_string()),
            password: "valid_password".to_string(),
        };
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_login_request_neither_username_nor_email_validation() {
        let invalid_request = LoginRequest {
            username: None,
            email: None,
            password: "valid_password".to_string(),
        };
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_login_request_invalid_email() {
        let invalid_request = LoginRequest {
            username: None,
            email: Some("invalid_email".to_string()),
            password: "valid_password".to_string(),
        };
        assert!(invalid_request.validate().is_err());
    }
}
