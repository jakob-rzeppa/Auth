use uuid::Uuid;

#[derive(Debug)]
pub enum PrivilegeError {
    EmptyUuid,
    EmptyName,
    NameTooLong,
    InvalidNameFormat,
    EmptyDescription,
}

#[derive(Debug)]
pub struct Privilege {
    id: Uuid,
    name: String,
    description: String,
}

impl Privilege {
    pub fn new(id: Uuid, name: String, description: String) -> Result<Self, PrivilegeError> {
        if id.is_nil() {
            return Err(PrivilegeError::EmptyUuid);
        }

        if name.is_empty() {
            return Err(PrivilegeError::EmptyName);
        }
        if name.len() > 255 {
            return Err(PrivilegeError::NameTooLong);
        }
        if !name.chars().all(|c| c.is_ascii_uppercase() || c == '_') {
            return Err(PrivilegeError::InvalidNameFormat);
        }
        if name.starts_with('_') || name.ends_with('_') {
            return Err(PrivilegeError::InvalidNameFormat);
        }
        if name.contains("__") {
            return Err(PrivilegeError::InvalidNameFormat);
        }

        if description.is_empty() {
            return Err(PrivilegeError::EmptyDescription);
        }

        Ok(Privilege {
            id,
            name: name.to_string(),
            description: description.to_string(),
        })
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_privilege_with_valid_inputs() {
        let id = Uuid::new_v4();
        let name = "READ_USERS".to_string();
        let description = "Can read user data".to_string();

        let privilege = Privilege::new(id, name, description);

        assert!(privilege.is_ok());
        let p = privilege.unwrap();
        assert_eq!(p.id(), &id);
        assert_eq!(p.name(), "READ_USERS");
        assert_eq!(p.description(), "Can read user data");
    }

    #[test]
    fn new_rejects_empty_uuid() {
        let id = Uuid::nil();
        let name = "READ_USERS".to_string();
        let description = "Can read user data".to_string();

        let result = Privilege::new(id, name, description);

        assert!(result.is_err());
        assert!(matches!(result, Err(PrivilegeError::EmptyUuid)));
    }

    #[test]
    fn new_rejects_empty_name() {
        let id = Uuid::new_v4();
        let name = String::new();
        let description = "Can read user data".to_string();

        let result = Privilege::new(id, name, description);

        assert!(result.is_err());
        assert!(matches!(result, Err(PrivilegeError::EmptyName)));
    }

    #[test]
    fn new_rejects_name_with_255_chars() {
        let id = Uuid::new_v4();
        let name = "A".repeat(255);
        let description = "Can read user data".to_string();

        let result = Privilege::new(id, name, description);

        assert!(result.is_ok());
    }

    #[test]
    fn new_rejects_name_exceeding_255_chars() {
        let id = Uuid::new_v4();
        let name = "A".repeat(256);
        let description = "Can read user data".to_string();

        let result = Privilege::new(id, name, description);

        assert!(result.is_err());
        assert!(matches!(result, Err(PrivilegeError::NameTooLong)));
    }

    #[test]
    fn new_rejects_name_with_lowercase_letters() {
        let id = Uuid::new_v4();
        let name = "Read_Users".to_string();
        let description = "Can read user data".to_string();

        let result = Privilege::new(id, name, description);

        assert!(result.is_err());
        assert!(matches!(result, Err(PrivilegeError::InvalidNameFormat)));
    }

    #[test]
    fn new_rejects_name_with_numbers() {
        let id = Uuid::new_v4();
        let name = "READ_USERS_123".to_string();
        let description = "Can read user data".to_string();

        let result = Privilege::new(id, name, description);

        assert!(result.is_err());
        assert!(matches!(result, Err(PrivilegeError::InvalidNameFormat)));
    }

    #[test]
    fn new_rejects_name_with_special_characters() {
        let id = Uuid::new_v4();
        let name = "READ-USERS".to_string();
        let description = "Can read user data".to_string();

        let result = Privilege::new(id, name, description);

        assert!(result.is_err());
        assert!(matches!(result, Err(PrivilegeError::InvalidNameFormat)));
    }

    #[test]
    fn new_accepts_name_with_underscores() {
        let id = Uuid::new_v4();
        let name = "READ_USER_DATA".to_string();
        let description = "Can read user data".to_string();

        let result = Privilege::new(id, name, description);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().name(), "READ_USER_DATA");
    }

    #[test]
    fn new_rejects_name_with_two_consecutive_underscores() {
        let id = Uuid::new_v4();
        let name = "READ__DATA".to_string();
        let description = "Can read user data".to_string();

        let result = Privilege::new(id, name, description);

        assert!(result.is_err());
        assert!(matches!(result, Err(PrivilegeError::InvalidNameFormat)));
    }

    #[test]
    fn new_rejectes_name_with_multiple_consecutive_underscores() {
        let id = Uuid::new_v4();
        let name = "READ___DATA".to_string();
        let description = "Can read user data".to_string();

        let result = Privilege::new(id, name, description);

        assert!(result.is_err());
        assert!(matches!(result, Err(PrivilegeError::InvalidNameFormat)));
    }

    #[test]
    fn new_rejects_name_with_leading_underscore() {
        let id = Uuid::new_v4();
        let name = "_READ_DATA".to_string();
        let description = "Can read user data".to_string();

        let result = Privilege::new(id, name, description);

        assert!(result.is_err());
        assert!(matches!(result, Err(PrivilegeError::InvalidNameFormat)));
    }

    fn new_rejects_name_with_trailing_underscore() {
        let id = Uuid::new_v4();
        let name = "READ_DATA_".to_string();
        let description = "Can read user data".to_string();

        let result = Privilege::new(id, name, description);

        assert!(result.is_err());
        assert!(matches!(result, Err(PrivilegeError::InvalidNameFormat)));
    }

    #[test]
    fn new_accepts_single_character_uppercase_name() {
        let id = Uuid::new_v4();
        let name = "A".to_string();
        let description = "Can read user data".to_string();

        let result = Privilege::new(id, name, description);

        assert!(result.is_ok());
    }

    #[test]
    fn new_rejects_empty_description() {
        let id = Uuid::new_v4();
        let name = "READ_USERS".to_string();
        let description = String::new();

        let result = Privilege::new(id, name, description);

        assert!(result.is_err());
        assert!(matches!(result, Err(PrivilegeError::EmptyDescription)));
    }

    #[test]
    fn new_accepts_single_character_description() {
        let id = Uuid::new_v4();
        let name = "READ_USERS".to_string();
        let description = "X".to_string();

        let result = Privilege::new(id, name, description);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().description(), "X");
    }

    #[test]
    fn new_accepts_long_description() {
        let id = Uuid::new_v4();
        let name = "READ_USERS".to_string();
        let description = "This is a very long description that contains many characters and explains what this privilege does in great detail".to_string();

        let result = Privilege::new(id, name, description);

        assert!(result.is_ok());
    }

    #[test]
    fn new_accepts_description_with_special_characters() {
        let id = Uuid::new_v4();
        let name = "READ_USERS".to_string();
        let description = "Can read user data: name, email, phone #123".to_string();

        let result = Privilege::new(id, name, description);

        assert!(result.is_ok());
    }

    #[test]
    fn new_rejects_empty_uuid_before_validating_other_fields() {
        let id = Uuid::nil();
        let name = String::new();
        let description = String::new();

        let result = Privilege::new(id, name, description);

        assert!(matches!(result, Err(PrivilegeError::EmptyUuid)));
    }

    #[test]
    fn new_rejects_empty_name_before_validating_description() {
        let id = Uuid::new_v4();
        let name = String::new();
        let description = String::new();

        let result = Privilege::new(id, name, description);

        assert!(matches!(result, Err(PrivilegeError::EmptyName)));
    }

    #[test]
    fn new_rejects_name_too_long_before_validating_format() {
        let id = Uuid::new_v4();
        let name = "A".repeat(256);
        let description = String::new();

        let result = Privilege::new(id, name, description);

        assert!(matches!(result, Err(PrivilegeError::NameTooLong)));
    }

    #[test]
    fn new_rejects_invalid_format_before_validating_description() {
        let id = Uuid::new_v4();
        let name = "read_users".to_string();
        let description = String::new();

        let result = Privilege::new(id, name, description);

        assert!(matches!(result, Err(PrivilegeError::InvalidNameFormat)));
    }
}
