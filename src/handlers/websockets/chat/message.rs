use super::{ChatManager, ChatSession};
use actix::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub enum MessageType {
    TYPING,
    TEXT,
    MATCH,
    UNMATCH,
}

impl Default for MessageType {
    fn default() -> Self {
        Self::TEXT
    }
}

#[derive(Message, Serialize, Deserialize, Debug)]
#[rtype(result = "()")]
pub struct ChatMessage {
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,
    sender_id: Uuid,
    receiver_id: Uuid,
    message: String,
    message_type: MessageType,
}

impl Handler<ChatMessage> for ChatManager {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, _: &mut Self::Context) -> Self::Result {
        if let Some(recv) = self.sessions.get(&msg.receiver_id) {
            recv.do_send(Message(json!(msg).to_string()));
        }
    }
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct Typing {
    sender_id: Uuid,
    receiver_id: Uuid,
    typing: bool,
}

impl Handler<Typing> for ChatManager {
    type Result = ();

    fn handle(&mut self, msg: Typing, _: &mut Self::Context) -> Self::Result {
        if let Some(recv) = self.sessions.get(&msg.receiver_id) {
            recv.do_send(Message(json!(msg).to_string()));
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Addr<ChatSession>,
    pub id: Uuid,
}

impl Handler<Connect> for ChatManager {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        self.add_session(msg.id, msg.addr);
        for (id, recv) in self.sessions.clone() {
            if id != msg.id {
                recv.do_send(Message(format!("{} connected", msg.id)));
            }
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
}

impl Handler<Disconnect> for ChatManager {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.id);
        for (_, recv) in self.sessions.clone() {
            recv.do_send(Message(format!("{} disconnected", msg.id)));
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Match {
    sender_id: Uuid,
    receiver_id: Uuid,
}

impl Handler<Match> for ChatManager {
    type Result = ();

    fn handle(&mut self, msg: Match, _: &mut Self::Context) -> Self::Result {
        if let Some(recv) = self.sessions.get(&msg.receiver_id) {
            recv.do_send(Message(format!("Chat started with {}", msg.sender_id)));
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Unmatch {
    sender_id: Uuid,
    receiver_id: Uuid,
}

impl Handler<Unmatch> for ChatManager {
    type Result = ();

    fn handle(&mut self, msg: Unmatch, _: &mut Self::Context) -> Self::Result {
        if let Some(recv) = self.sessions.get(&msg.receiver_id) {
            recv.do_send(Message(format!("Chat ended by {}", msg.sender_id)));
        }
    }
}
