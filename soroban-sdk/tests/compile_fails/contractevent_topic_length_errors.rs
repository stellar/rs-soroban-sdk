use soroban_sdk::{contractevent, Address};

#[contractevent(topics = ["a_topic_that_is_longer_than_the_limit"])]
pub struct ExplicitTopic {
    pub from: Address,
}

// The name fits the event name limit, but its default topic, the name in snake
// case, does not fit the topic limit.
#[contractevent]
pub struct AbCdEfGhIjKlMnOpQrStUvWx {
    pub from: Address,
}

#[contractevent(topics = ["fits"])]
pub struct Fits {
    pub from: Address,
}

fn main() {}
