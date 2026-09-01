use privilege::Privilege;
use uuid::Uuid;

#[derive(Debug)]
pub enum UserError {
    EmptyId,
    InvalidEmail,
}

pub struct User {
    id: Uuid,

    email: String,

    privileges: Vec<Privilege>,
}

impl User {
    pub fn new(id: Uuid, email: String, privileges: Vec<Privilege>) -> Result<Self, UserError> {
        if id.is_nil() {
            return Err(UserError::EmptyId);
        }

        if email.is_empty() || !email.contains('@') {
            return Err(UserError::InvalidEmail);
        }

        Ok(Self {
            id,
            email,
            privileges,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn privileges(&self) -> &[Privilege] {
        &self.privileges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_nil_id() {
        let result = User::new(Uuid::nil(), "user@example.com".to_string(), vec![]);
        assert!(matches!(result, Err(UserError::EmptyId)));
    }

    #[test]
    fn rejects_empty_email() {
        let result = User::new(Uuid::new_v4(), "".to_string(), vec![]);
        assert!(matches!(result, Err(UserError::InvalidEmail)));
    }

    #[test]
    fn rejects_email_without_at_symbol() {
        let result = User::new(Uuid::new_v4(), "invalidemail.com".to_string(), vec![]);
        assert!(matches!(result, Err(UserError::InvalidEmail)));
    }

    #[test]
    fn creates_user_with_valid_inputs() {
        let id = Uuid::new_v4();
        let email = "user@example.com".to_string();
        let privileges = vec![];

        let result = User::new(id, email.clone(), privileges.clone());

        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.id(), id);
        assert_eq!(user.email(), &email);
        assert_eq!(user.privileges(), &privileges);
    }

    #[test]
    fn accepts_valid_email_formats() {
        let id = Uuid::new_v4();
        let valid_emails = vec!["a@b.com", "test+tag@example.co.uk", "user.name@domain.com"];

        for email in valid_emails {
            let result = User::new(id, email.to_string(), vec![]);
            assert!(result.is_ok(), "should accept email: {}", email);
        }
    }
}
