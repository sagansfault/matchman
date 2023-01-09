pub mod data {
    use std::collections::HashMap;

    pub struct Data {
        pub fights: Vec<super::fight::Fight>,
        pub pools: HashMap<super::user::MMUser, String>
    }
}

pub mod user {
    use serenity::model::prelude::UserId;

    #[derive(Hash, Eq, Clone)]
    pub struct MMUser {
        pub id: UserId,
        pub status: Status
    }

    impl PartialEq for MMUser {
        fn eq(&self, other: &Self) -> bool {
            self.id.0 == other.id.0
        }
    }

    #[derive(Hash, PartialEq, Eq, Clone, Copy)]
    pub enum Status {
        Fighting, Queued
    }
}

pub mod fight {
    use super::user::MMUser;
    pub struct Fight {
        pub p1: MMUser,
        pub p2: MMUser
    }
}