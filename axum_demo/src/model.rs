use std::sync::Arc;

use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;

use crate::prelude::{Result, Error};


#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>
}

// constructor
impl ModelController {
    pub fn new() -> Result<Self>  {
        Ok(Self {
            tickets_store: Arc::default()
        })
    }
}

// crud
impl ModelController {
    pub async fn create_ticket(&self, t: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().await;
        let id = store.len() as u64; 
        let ticket = Ticket {
            id,
            title: t.title,
        };
        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_ticket(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().await;
        let tickets = store.iter().filter_map(|t| t.clone()).collect();
        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().await;
        let ticket = store.get_mut(id as usize).and_then(|t| t.take());
        ticket.ok_or(Error::TicketDeleteIdNotFound { id: id })
    }

}