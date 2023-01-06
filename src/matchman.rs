pub mod data {
    use dashmap::DashMap;

    pub struct Data {
        fights: DashMap<u64, Vec<super::fight::Fight>>,
        queues: DashMap<u64, Vec<super::user::MMUser>>
    }
}

pub mod user {
    pub struct MMUser {
        pub id: u64,
        pub status: Status
    }

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