use uuid::Uuid;

use crate::{domain::entity::role::Role, persistence::roles::store::roles};

pub fn find_roles_by_ids(role_ids: &[Uuid]) -> Vec<Role> {
    let res: Vec<Role> = roles()
        .iter()
        .filter(|role| role_ids.contains(role.id()))
        .cloned()
        .collect();

    if res.len() != role_ids.len() {
        let found_ids: Vec<Uuid> = res.iter().map(|role| role.id().clone()).collect();
        let missing_ids: Vec<Uuid> = role_ids
            .iter()
            .filter(|id| !found_ids.contains(id))
            .cloned()
            .collect();
        eprintln!(
            "Some role ids were not found in the role store: {:?}",
            missing_ids
        );
    }

    res
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use uuid::Uuid;

    use crate::persistence::roles::store::roles_fake;

    use super::*;

    static TEST_ROLES: LazyLock<Vec<Role>> = LazyLock::new(|| {
        vec![
            Role::new(
                Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
                "Admin".to_string(),
                "Administrator role".to_string(),
            )
            .unwrap(),
            Role::new(
                Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
                "User".to_string(),
                "Regular user role".to_string(),
            )
            .unwrap(),
            Role::new(
                Uuid::parse_str("00000000-0000-0000-0000-000000000003").unwrap(),
                "Guest".to_string(),
                "Temporary guest".to_string(),
            )
            .unwrap(),
        ]
    });

    #[test]
    fn test_find_role_by_id() {
        roles_fake().setup(|| &TEST_ROLES);

        let roles = find_roles_by_ids(&[
            Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
            Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
        ]);

        assert_eq!(roles.len(), 2);
        assert_eq!(
            roles.iter().map(|r| r.id()).collect::<Vec<_>>(),
            vec![
                &Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
                &Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
            ]
        );
    }

    #[test]
    fn test_find_role_by_id_with_nonexistent_id() {
        roles_fake().setup(|| &TEST_ROLES);

        let roles = find_roles_by_ids(&[
            Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
            Uuid::parse_str("00000000-0000-0000-0000-000000000004").unwrap(), // Non-existent ID
        ]);

        assert_eq!(roles.len(), 1);
        assert_eq!(
            roles[0].id(),
            &Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()
        );
    }
}
