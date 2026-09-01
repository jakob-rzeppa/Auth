use std::fmt::Display;

use uuid::Uuid;

#[derive(PartialEq, Debug, Clone)]
pub enum Privilege {}

impl Display for Privilege {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl Privilege {
    pub fn id(&self) -> Uuid {
        todo!()
    }

    pub fn name(&self) -> &str {
        todo!()
    }

    pub fn description(&self) -> &str {
        todo!()
    }
}
