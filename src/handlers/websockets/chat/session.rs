use super::message::Message;
use super::{
    message::{ChatMessage, Connect, Disconnect},
    ChatManager,
};
use actix::prelude::*;
use actix_web_actors::ws::{self, WebsocketContext};
use uuid::Uuid;

pub struct ChatSession {
    pub id: Uuid,
    pub mgr: Addr<ChatManager>,
}

impl Actor for ChatSession {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.mgr.do_send(Connect {
            addr: ctx.address(),
            id: self.id.clone(),
        });
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.mgr.do_send(Disconnect {
            id: self.id.clone(),
        });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                let x: ChatMessage = serde_json::from_str(&text).unwrap();
                self.mgr.do_send(x);
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Ping(_)) => ctx.pong("pong".as_bytes()),
            Ok(ws::Message::Pong(_)) => {}
            Ok(ws::Message::Nop) => {}
            Ok(ws::Message::Close(reason)) => ctx.close(reason),
            Ok(ws::Message::Continuation(_)) => {}
            Err(_) => todo!(),
        };
    }
}

impl ChatSession {
    pub fn new(id: Uuid, mgr: Addr<ChatManager>) -> Self {
        ChatSession { id, mgr }
    }
}

impl Handler<Message> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}
