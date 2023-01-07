pub mod data {
    use std::collections::HashSet;

    use dashmap::DashMap;

    pub struct Data {
        fights: DashMap<u64, Vec<super::fight::Fight>>,
        pools: DashMap<u64, HashSet<super::user::MMUser>>
    }
}

pub mod user {

    #[derive(Hash, Eq)]
    pub struct MMUser {
        pub id: u64,
        pub status: Status
    }

    impl PartialEq for MMUser {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    #[derive(Hash, PartialEq, Eq)]
    pub enum Status {
        Fighting, Queued
    }
}

pub mod game {
    pub trait MMGame {
        // command parser, message sender for each game etc
        fn gmae_type() -> MMGameType;
    }
    
    pub enum MMGameType {
        GuiltyGearStrive, SkullGirls
    }
}

pub mod fight {
    use super::user::MMUser;
    pub struct Fight {
        pub p1: MMUser,
        pub p2: MMUser
    }
}