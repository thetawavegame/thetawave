use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub enum TutorialLessonType {
    Movement,
    Attack,
    SpecialAbility,
}
