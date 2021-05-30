pub struct Transition {
    state: String,
    next: String,
}

pub struct FSM {
    transitions: Vec<Transition>,
}

pub fn fsm_traffic() -> FSM {
    FSM {
        transitions: vec![
            Transition {
                state: "red".to_string(),
                next: "green".to_string(),
            },
            Transition {
                state: "green".to_string(),
                next: "yellow".to_string(),
            },
            Transition {
                state: "yellow".to_string(),
                next: "red".to_string(),
            },
        ],
    }
}

pub fn format_fsm(fsm: FSM) -> String {
    fsm.transitions
        .iter()
        .map(format_transition)
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_transition(t: &Transition) -> String {
    format!("{}->{}", t.state, t.next)
}
