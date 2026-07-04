use super::context::GameContext;
use super::events::GameEvent;
use super::types::*;

pub fn map_decision_to_events(
    scenario: &Scenario,
    option_index: usize,
    context: &GameContext,
) -> Vec<GameEvent> {
    let mut events = Vec::new();

    let option = match scenario.player_options.get(option_index) {
        Some(o) => o,
        None => return events,
    };

    // ponytail: heuristic consequence mapping — LLM refines in next scenario
    let label_lower = option.label.to_lowercase();
    let desc_lower = option.description.to_lowercase();

    // DEFCON changes
    if desc_lower.contains("escalat") || label_lower.contains("scramble") || label_lower.contains("defcon") {
        let target = context.defcon_level.saturating_sub(1).max(1);
        events.push(GameEvent::DefconChange(target));
    } else if desc_lower.contains("de-escalat") || desc_lower.contains("diplomatic") || label_lower.contains("back-channel") {
        let target = context.defcon_level.saturating_add(1).min(5);
        events.push(GameEvent::DefconChange(target));
    }

    // threat detection from scenario regions
    for region in &scenario.affected_regions {
        if scenario.threat_level == ThreatLevel::Critical || scenario.threat_level == ThreatLevel::High {
            events.push(GameEvent::ThreatDetected {
                location: region_to_coords(region),
                severity: scenario.threat_level,
            });
        }
    }

    // comms from scenario
    for comm in &scenario.comms {
        events.push(GameEvent::CommReceived(comm.clone()));
    }

    // world state shifts based on action type
    let (eco, mil, pol) = if desc_lower.contains("escalat") || desc_lower.contains("scramble") || desc_lower.contains("military") {
        (-0.05, 0.1, 0.05)
    } else if desc_lower.contains("diplomatic") || desc_lower.contains("de-escalat") || desc_lower.contains("negotiate") {
        (0.05, -0.1, -0.05)
    } else if desc_lower.contains("sanction") || desc_lower.contains("economic") {
        (-0.1, 0.0, 0.05)
    } else {
        (0.0, 0.05, 0.0)
    };
    events.push(GameEvent::WorldStateUpdate { economic: eco as f32, military: mil as f32, political: pol as f32 });

    // record decision
    events.push(GameEvent::PlayerDecision {
        scenario_id: scenario.id,
        option_index,
    });

    events
}

pub fn region_to_coords(region: &str) -> (f32, f32) {
    let r = region.to_lowercase();
    // ponytail: rough center coords for common regions, fallback to 0,0
    if r.contains("arctic") { (75.0, 0.0) }
    else if r.contains("baltic") { (57.0, 20.0) }
    else if r.contains("taiwan") || r.contains("south china") { (22.0, 115.0) }
    else if r.contains("kaliningrad") { (54.7, 20.5) }
    else if r.contains("barents") { (72.0, 35.0) }
    else if r.contains("kamchatka") { (56.0, 160.0) }
    else if r.contains("pacific") { (30.0, -170.0) }
    else if r.contains("europe") || r.contains("eu") { (50.0, 10.0) }
    else if r.contains("okinawa") { (26.3, 127.8) }
    else if r.contains("fujian") { (26.0, 118.0) }
    else { (0.0, 0.0) }
}
