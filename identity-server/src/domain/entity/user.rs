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

    password_hash: String,
    has_temporary_password: bool,

    privileges: Vec<Privilege>,
}

impl User {
    pub fn new(
        id: Uuid,
        email: String,
        password_hash: String,
        has_temporary_password: bool,
        privileges: Vec<Privilege>,
    ) -> Result<Self, UserError> {
        if id.is_nil() {
            return Err(UserError::EmptyId);
        }

        if email.is_empty() || !email.contains('@') {
            return Err(UserError::InvalidEmail);
        }

        Ok(Self {
            id,
            email,
            password_hash,
            has_temporary_password,
            privileges,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn set_email(&mut self, email: String) -> Result<(), UserError> {
        if email.is_empty() || !email.contains('@') {
            return Err(UserError::InvalidEmail);
        }

        self.email = email;

        Ok(())
    }

    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }

    pub fn set_password_hash(&mut self, password_hash: String) {
        self.password_hash = password_hash;
    }

    pub fn has_temporary_password(&self) -> bool {
        self.has_temporary_password
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
        let result = User::new(
            Uuid::nil(),
            "user@example.com".to_string(),
            "password_hash".to_string(),
            false,
            vec![],
        );
        assert!(matches!(result, Err(UserError::EmptyId)));
    }

    #[test]
    fn rejects_empty_email() {
        let result = User::new(
            Uuid::new_v4(),
            "".to_string(),
            "password_hash".to_string(),
            false,
            vec![],
        );
        assert!(matches!(result, Err(UserError::InvalidEmail)));
    }

    #[test]
    fn rejects_email_without_at_symbol() {
        let result = User::new(
            Uuid::new_v4(),
            "invalidemail.com".to_string(),
            "password_hash".to_string(),
            false,
            vec![],
        );
        assert!(matches!(result, Err(UserError::InvalidEmail)));
    }

    #[test]
    fn creates_user_with_valid_inputs() {
        let id = Uuid::new_v4();
        let email = "user@example.com".to_string();
        let privileges = vec![];

        let result = User::new(
            id,
            email.clone(),
            "password_hash".to_string(),
            false,
            privileges.clone(),
        );

        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.id(), id);
        assert_eq!(user.email(), &email);
        assert_eq!(user.privileges(), &privileges);
    }

    #[test]
    fn set_email_updates_email_with_valid_input() {
        let mut user = User::new(
            Uuid::new_v4(),
            "old@example.com".to_string(),
            "password_hash".to_string(),
            false,
            vec![],
        )
        .unwrap();

        let result = user.set_email("new@example.com".to_string());

        assert!(result.is_ok());
        assert_eq!(user.email(), "new@example.com");
    }

    #[test]
    fn set_email_rejects_empty_email() {
        let mut user = User::new(
            Uuid::new_v4(),
            "old@example.com".to_string(),
            "password_hash".to_string(),
            false,
            vec![],
        )
        .unwrap();

        let result = user.set_email("".to_string());

        assert!(matches!(result, Err(UserError::InvalidEmail)));
        assert_eq!(user.email(), "old@example.com");
    }

    #[test]
    fn set_email_rejects_email_without_at_symbol() {
        let mut user = User::new(
            Uuid::new_v4(),
            "old@example.com".to_string(),
            "password_hash".to_string(),
            false,
            vec![],
        )
        .unwrap();

        let result = user.set_email("invalidemail.com".to_string());

        assert!(matches!(result, Err(UserError::InvalidEmail)));
        assert_eq!(user.email(), "old@example.com");
    }

    #[test]
    fn accepts_valid_email_formats() {
        let id = Uuid::new_v4();
        let valid_emails = vec!["a@b.com", "test+tag@example.co.uk", "user.name@domain.com"];

        for email in valid_emails {
            let result = User::new(
                id,
                email.to_string(),
                "password_hash".to_string(),
                false,
                vec![],
            );
            assert!(result.is_ok(), "should accept email: {}", email);
        }
    }
}
