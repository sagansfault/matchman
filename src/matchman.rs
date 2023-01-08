pub mod data {
    use std::collections::HashMap;

    pub struct Data<'a> {
        pub fights: Vec<super::fight::Fight<'a>>,
        pub pools: HashMap<super::user::MMUser<'a>, String>
    }
}

pub mod user {
    use serenity::model::user::User;


    #[derive(Hash, Eq, Clone)]
    pub struct MMUser<'a> {
        pub user: &'a User,
        pub status: Status
    }

    impl<'a> PartialEq for MMUser<'a> {
        fn eq(&self, other: &Self) -> bool {
            self.user.id.0 == other.user.id.0
        }
    }

    #[derive(Hash, PartialEq, Eq, Clone, Copy)]
    pub enum Status {
        Fighting, Queued
    }
}

pub mod fight {
    use super::user::MMUser;
    pub struct Fight<'a> {
        pub p1: MMUser<'a>,
        pub p2: MMUser<'a>
    }
}