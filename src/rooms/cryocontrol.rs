use crate::game_event::GameEventType;
use crate::room::Room;
use crate::timer::{Timer, TimerType};
use crate::EventQueue;
use crate::{Action, ActionHandled, State};

#[derive(Debug)]
pub struct Cryocontrol {
    pub visited: bool,
    pub lever: bool,
    pub opened: bool,
}

impl Cryocontrol {
    pub fn new() -> Cryocontrol {
        Cryocontrol {
            visited: false,
            lever: false,
            opened: false,
        }
    }
}

impl Room for Cryocontrol {
    fn handle_action(
        &mut self,
        state: &mut State,
        event_queue: &mut EventQueue,
        action: &Action,
    ) -> ActionHandled {
        match action {
            Action::UseTerminal => {
                if let Some(enemy) = state.get_current_enemy(state.current_room) {
                    event_queue.schedule_action(Action::Message(
                        format!("The {:?} is blocking you from getting to the terminal.", enemy.get_enemy_type()),
                        GameEventType::Failure
                    ));
                    return ActionHandled::Handled;
                }

                let time_to_reboot = 20_000;
                event_queue.schedule_action(Action::Message(
                    format!("The AI's voice is slowly dying. \"Rebooting System in {} seconds... boot process, life support systems will be offline. All passengers, please enter cryosleep caskets immediately.", time_to_reboot / 1000),
                    GameEventType::Success,
                ));
                event_queue.schedule_timer(Timer::new(
                    TimerType::Reboot,
                    "Reboot countdown",
                    0,
                    time_to_reboot,
                    Action::Rebooted,
                    true,
                ));
                ActionHandled::Handled
            }
            Action::OpenCryoControl => {
                if !self.opened {
                    self.opened = true;
                    event_queue.schedule_action(Action::Message(
                        String::from("You open the cryo control door."),
                        GameEventType::Success,
                    ));

                    ActionHandled::Handled
                } else {
                    event_queue.schedule_action(Action::Message(
                        String::from("The cryo control door is already open."),
                        GameEventType::Failure,
                    ));

                    ActionHandled::Handled
                }
            }
            _ => ActionHandled::NotHandled,
        }
    }

    fn is_opened(&self) -> bool {
        self.opened
    }

    fn open(&mut self) {
        self.opened = true
    }

    fn visit(&mut self) {
        self.visited = true;
    }

    fn is_visited(&self) -> bool {
        self.visited
    }
}
