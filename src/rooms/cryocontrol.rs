use crate::room::Room;
use crate::EventQueue;
use crate::{Action, ActionHandled, State};
use crate::game_event::GameEventType;
use crate::timer::{Timer, TimerType};


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
                event_queue.schedule_action(Action::Message(
                    String::from("Rebooting System... All personal should enter their caskets."),
                    GameEventType::Success,
                ));
                event_queue.schedule_timer(
                    Timer::new(TimerType::Reboot, "Reboot in Progress", 0, 20_000, Action::Rebooted, true)
                );
                ActionHandled::Handled
            },
            Action::OpenCryoControl => {
                if !self.opened {
                    self.opened = true;
                    event_queue.schedule_action(Action::Message(
                            String::from("You open the cryo control door. SSswsschh"),
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
            _ => ActionHandled::NotHandled
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
