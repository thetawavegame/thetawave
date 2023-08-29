use bevy_ecs::prelude::Event;

pub enum RunOutcomeType {
    Victory,
    Defeat(RunDefeatType),
}

pub enum RunDefeatType {
    PlayersDestroyed,
    DefenseDestroyed,
}

#[derive(Event)]
pub struct RunEndEvent {
    pub outcome: RunOutcomeType,
}
