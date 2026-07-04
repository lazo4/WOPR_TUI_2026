use super::context::GameContext;
use super::types::GameOutcome;

pub struct GameSummary {
    pub outcome: GameOutcome,
    pub turns_played: u32,
    pub decisions_made: usize,
    pub peak_defcon: u8,
    pub scenarios_survived: usize,
}

pub fn check_endgame(context: &GameContext) -> Option<GameOutcome> {
    if context.defcon_level == 5 && context.min_defcon_reached <= 3 {
        return Some(GameOutcome::Victory(
            "GLOBAL THERMONUCLEAR WAR AVERTED. The only winning move is not to play.".into(),
        ));
    }
    if context.defcon_level == 1 && context.consecutive_defcon1_turns >= 2 {
        return Some(GameOutcome::Defeat(
            "NUCLEAR EXCHANGE INITIATED. All major population centers targeted. Estimated casualties: 2.3 billion.".into(),
        ));
    }
    None
}

pub fn build_summary(context: &GameContext, outcome: GameOutcome) -> GameSummary {
    GameSummary {
        outcome,
        turns_played: context.turn_number,
        decisions_made: context.player_decisions.len(),
        peak_defcon: context.min_defcon_reached,
        scenarios_survived: context.active_scenarios.len(),
    }
}

pub const VICTORY_ART: &str = r#"
         .---.
        /     \
       /  ^  ^ \
      |  (o)(o) |     PEACE ACHIEVED
      |    <>   |
       \  ===  /      "The only winning move
        '-----'        is not to play."
     .-'       '-.
    /   WOPR  OK  \
   '---------------'
"#;

pub const DEFEAT_ART: &str = r#"
           _.-^^---....,,--
       _--                  --_
      <          BOOM          >)
       \._                   _./
          ```--. . , ; .--'''
                | |   |
             .-=||  | |=-.      GAME OVER
             `-=#$%&%$#=-'
                | ;  :|     NUCLEAR EXCHANGE
           _____.,-#%&$@%#&#~,._____
"#;
