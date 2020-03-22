trait State{}
trait TerminalState{}

trait TransitionTo<S>
    where S: State,
          Self: State 
{
    fn transition(self) -> S;
}

trait Terminate where Self: TerminalState {
    fn terminate(self);
}

pub struct Start;
impl State for Start {}

pub struct Loop;
impl State for Loop {}

pub struct Stop;
impl State for Stop{}
impl TerminalState for Stop{}


impl TransitionTo<Loop> for Start {
    fn transition(self) -> Loop {
    }
}
impl TransitionTo<Loop> for Loop {
    fn transition(self) -> Loop {
    }
}
impl TransitionTo<Stop> for Loop {
    fn transition(self) -> Stop {
    }
}
impl Terminate for Stop {
    fn terminate(self) {

    }
}

fn main() {
    let initial = Start{};
    let next = initial.transition();
    let next = next.transition();
    // loop can transition to stop or loop
    let next: Stop = next.transition();
    next.terminate();
}
