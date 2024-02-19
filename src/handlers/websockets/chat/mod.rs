use actix::prelude::*;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws::WsResponseBuilder;
use std::{collections::HashMap, str::FromStr};
use uuid::Uuid;

mod session;
use session::ChatSession;

use crate::utils::jwt::get_claims_from_token;
mod message;

pub struct ChatManager {
    sessions: HashMap<Uuid, Addr<ChatSession>>,
}

impl Actor for ChatManager {
    type Context = Context<Self>;
}

impl ChatManager {
    pub fn new() -> Self {
        ChatManager {
            sessions: HashMap::new(),
        }
    }

    pub fn add_session(&mut self, id: Uuid, addr: Addr<ChatSession>) {
        self.sessions.insert(id, addr);
    }

    pub fn remove_session(&mut self, id: Uuid) {
        self.sessions.remove(&id);
    }

    pub async fn chat_route(
        req: HttpRequest,
        stream: web::Payload,
        data: web::Data<Addr<ChatManager>>,
    ) -> Result<HttpResponse, actix_web::Error> {
        let auth_token = req.query_string();
        let user_id =
            Uuid::from_str(&get_claims_from_token(auth_token.into()).unwrap().sub).unwrap();
        let chat_session = ChatSession::new(user_id, data.get_ref().clone());
        let builder =
            WsResponseBuilder::new(chat_session, &req, stream).protocols(&["echo-protocol"]);
        builder.start()
    }
}
