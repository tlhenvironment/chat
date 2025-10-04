use rand::{distr::Alphanumeric, Rng};
use tokio::sync::{mpsc::{self, Receiver}, oneshot};

pub enum StateMessage {
    Get(StateMessageGet),
}

pub enum StateMessageGet {
    GetName(oneshot::Sender<String>),
    GetRandomKey(oneshot::Sender<String>),
}

pub struct State {
    name: String,
    random_hash: String,
}

impl State {
    pub fn new(name: String) -> Self {
        let rng = rand::rng();
        let random_key = rng.sample_iter(&Alphanumeric).take(100).map(char::from).collect();

        State { name, random_hash: random_key }
    }

    pub fn eat_message(&self, message: StateMessage) {
        match message {
            StateMessage::Get(x) => {
                match x {
                    StateMessageGet::GetName(sender) => {
                        sender.send(self.name.clone()).unwrap();
                    },
                    StateMessageGet::GetRandomKey(sender) => {
                        sender.send(self.random_hash.clone()).unwrap();
                    },
                }
            },
        }
    }

    pub fn get_random_hash(&self) -> &String {
        &self.random_hash
    }
}

pub async fn state_loop(state: State, mut receiver: Receiver<StateMessage>) {
    loop {
        let message = receiver.recv().await.unwrap();
        state.eat_message(message);
    }
}