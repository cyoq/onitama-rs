use bevy::prelude::Entity;

#[derive(Debug, Clone)]
pub struct TextHandler {
    pub turn_text: Entity,
    pub guide_text: Entity,
    pub evaluation_text: Entity
}

#[derive(Debug, Clone)]
pub struct EvaluationResult {
    pub score: i32
}

impl Default for EvaluationResult {
    fn default() -> Self {
        Self { score: 0 }
    }
}

impl ToString for EvaluationResult {
    fn to_string(&self) -> String {
        format!("Evaluation result: {}", self.score)
    }
}