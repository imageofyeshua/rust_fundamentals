mod diet {
    const NUTRITIONIST: &str = "Nora Nutrition";

    pub fn ask_about_program() {
        println!("The nutritionist is {NUTRITIONIST}");
    }
}
mod cardio;
mod weightlifting;

use cardio::{CardioTool, Exercise as CardioExercise};
use weightlifting::Exercise as WeightliftingExercise;

#[derive(Debug)]
pub struct GymWorkout {
    cardio: CardioExercise,
    weightlifting: WeightliftingExercise,
}

impl GymWorkout {
    pub fn new() -> Self {
        diet::ask_about_program();
        cardio::ask_about_program();
        weightlifting::ask_about_program();

        Self {
            cardio: CardioExercise::new(String::from("Thursday"), CardioTool::Bike, 30),
            weightlifting: WeightliftingExercise::new(String::from("Bench Press"), 8),
        }
    }
}
