use crate::util::BoostPad;

pub static BALL_TYPES: [&str; 5] = [
    "Archetypes.Ball.Ball_Default",
    "Archetypes.Ball.Ball_Basketball",
    "Archetypes.Ball.Ball_Puck",
    "Archetypes.Ball.CubeBall",
    "Archetypes.Ball.Ball_Breakout",
];

pub static BOOST_TYPE: &str = "Archetypes.CarComponents.CarComponent_Boost";
pub static CAR_TYPE: &str = "Archetypes.Car.Car_Default";
pub static DODGE_TYPE: &str = "Archetypes.CarComponents.CarComponent_Dodge";
pub static DOUBLE_JUMP_TYPE: &str = "Archetypes.CarComponents.CarComponent_DoubleJump";
pub static GAME_TYPE: &str = "Archetypes.GameEvent.GameEvent_Soccar";
pub static JUMP_TYPE: &str = "Archetypes.CarComponents.CarComponent_Jump";
pub static PLAYER_REPLICATION_KEY: &str = "Engine.Pawn:PlayerReplicationInfo";
pub static PLAYER_TYPE: &str = "TAGame.Default__PRI_TA";

pub static BOOST_AMOUNT_KEY: &str = "TAGame.CarComponent_Boost_TA:ReplicatedBoostAmount";
pub static COMPONENT_ACTIVE_KEY: &str = "TAGame.CarComponent_TA:ReplicatedActive";
pub static DEMOLISH_GOAL_EXPLOSION_KEY: &str = "TAGame.Car_TA:ReplicatedDemolishGoalExplosion";
pub static IGNORE_SYNCING_KEY: &str = "TAGame.RBActor_TA:bIgnoreSyncing";
pub static LAST_BOOST_AMOUNT_KEY: &str = "TAGame.CarComponent_Boost_TA:ReplicatedBoostAmount.Last";
pub static PLAYER_NAME_KEY: &str = "Engine.PlayerReplicationInfo:PlayerName";
pub static RIGID_BODY_STATE_KEY: &str = "TAGame.RBActor_TA:ReplicatedRBState";
pub static SECONDS_REMAINING_KEY: &str = "TAGame.GameEvent_Soccar_TA:SecondsRemaining";
pub static TEAM_KEY: &str = "Engine.PlayerReplicationInfo:Team";
pub static UNIQUE_ID_KEY: &str = "Engine.PlayerReplicationInfo:UniqueId";
pub static VEHICLE_KEY: &str = "TAGame.CarComponent_TA:Vehicle";

pub static EMPTY_ACTOR_IDS: [boxcars::ActorId; 0] = [];

pub static BOOST_USED_PER_SECOND: f32 = 80.0 / 0.93;
pub static FRAMES_PER_SECOND: f32 = 30.0;

pub static MAX_DEMOLISH_KNOWN_FRAMES_PASSED: usize = 100;

pub const LARGE_BOOST_RADIUS: f32 = 208.0;
pub const SMALL_BOOST_RADIUS: f32 = 149.0;
pub const SMALL_BOOST_HEIGHT: f32 = 165.0;
pub const LARGE_BOOST_HEIGHT: f32 = 168.0;
pub const BOOST_COOLDOWN: f32 = 5.0;
pub const STARTING_BOOST_AMOUNT: f32 = 81.6;

pub static SMALL_BOOST_PADS: [BoostPad; 28] = [
    BoostPad {
        id: 0,
        x: 0.0,
        y: -4240.0,
    },
    BoostPad {
        id: 1,
        x: -1792.0,
        y: -4184.0,
    },
    BoostPad {
        id: 2,
        x: 1792.0,
        y: -4184.0,
    },
    BoostPad {
        id: 3,
        x: -940.0,
        y: -3308.07,
    },
    BoostPad {
        id: 4,
        x: 940.0,
        y: -3308.0,
    },
    BoostPad {
        id: 5,
        x: 0.0,
        y: -2816.0,
    },
    BoostPad {
        id: 6,
        x: -3584.0,
        y: -2484.0,
    },
    BoostPad {
        id: 7,
        x: 3584.0,
        y: -2484.0,
    },
    BoostPad {
        id: 8,
        x: -1788.0,
        y: -2300.0,
    },
    BoostPad {
        id: 9,
        x: 1788.0,
        y: -2300.0,
    },
    BoostPad {
        id: 10,
        x: -2048.0,
        y: -1036.0,
    },
    BoostPad {
        id: 11,
        x: 0.0,
        y: -1024.0,
    },
    BoostPad {
        id: 12,
        x: 2048.0,
        y: -1036.0,
    },
    BoostPad {
        id: 13,
        x: -1024.0,
        y: 0.0,
    },
    BoostPad {
        id: 14,
        x: 1024.0,
        y: 0.0,
    },
    BoostPad {
        id: 15,
        x: -2048.0,
        y: 1036.0,
    },
    BoostPad {
        id: 16,
        x: 0.0,
        y: 1024.0,
    },
    BoostPad {
        id: 17,
        x: 2048.0,
        y: 1036.0,
    },
    BoostPad {
        id: 18,
        x: -1788.0,
        y: 2300.0,
    },
    BoostPad {
        id: 19,
        x: 1788.0,
        y: 2300.0,
    },
    BoostPad {
        id: 20,
        x: -3584.0,
        y: 2484.0,
    },
    BoostPad {
        id: 21,
        x: 3584.0,
        y: 2484.0,
    },
    BoostPad {
        id: 22,
        x: 0.0,
        y: 2816.0,
    },
    BoostPad {
        id: 23,
        x: -940.0,
        y: 3310.0,
    },
    BoostPad {
        id: 24,
        x: 940.0,
        y: 3308.0,
    },
    BoostPad {
        id: 25,
        x: -1792.0,
        y: 4184.0,
    },
    BoostPad {
        id: 26,
        x: 1792.0,
        y: 4184.0,
    },
    BoostPad {
        id: 27,
        x: 0.0,
        y: 4240.0,
    },
];

/// Large boost pad definitions by in-game location
pub static LARGE_BOOST_PADS: [BoostPad; 6] = [
    BoostPad {
        id: 28,
        x: 3072.0,
        y: -4096.0,
    },
    BoostPad {
        id: 29,
        x: -3072.0,
        y: -4096.0,
    },
    BoostPad {
        id: 30,
        x: 3584.0,
        y: 0.0,
    },
    BoostPad {
        id: 31,
        x: -3584.0,
        y: 0.0,
    },
    BoostPad {
        id: 32,
        x: 3072.0,
        y: 4096.0,
    },
    BoostPad {
        id: 33,
        x: -3072.0,
        y: 4096.0,
    },
];
