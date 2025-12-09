use iced::{ Element, Renderer };
use std::fmt::Debug;

// Die type enum for picklist
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DieType {
    D20,
    D12,
    D10,
    D8,
    D6,
    D4,
    D100, // Percentile die
}

/* Implementing the Display trait so the picklist knows how to display the 
enum values as strings in the UI */
impl fmt::Display for DieType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            DieType::D20 => "D20",
            DieType::D12 => "D12",
            DieType::D10 => "D10",
            DieType::D8  => "D8",
            DieType::D6  => "D6",
            DieType::D4  => "D4",
            DieType::D100 => "D100",
        })
    }
}

// Mode Select Enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModeSelect {
    DieRoller,
    AbilityScoreRoll,
}

impl fmt::Display for ModeSelect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            ModeSelect::DieRoller => "Die Roller",
            ModeSelect::AbilityScoreRoll => "Ability Score Roll (4d6 drop lowest)",
        })
    }
}

// Main Application State, Model
#[derive(Debug, Clone)]
pub struct DieRollerApp {
    // Input Fields
    pub die_type_selection: DieType,
    pub mode_selection: ModeSelect,

    // String for the input, will be parsed when we roll
    pub number_of_dice_input: String,

    // Output / History
    pub roll_history: Vec<String>,
}

impl Default for DiceRoller {
    fn default() -> Self {
        DiceRoller {
            die_type_selection: DieType::D20,
            mode_selection: RollMode::default(),
            num_dice_input: "1".to_string(),
            roll_history: Vec::new(),
        }
    }
}