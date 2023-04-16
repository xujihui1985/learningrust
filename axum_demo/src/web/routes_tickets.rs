use axum::routing::{post,get, delete};
use axum::{Json, Router};
use axum::extract::{State, Path};

use crate::ctx::Ctx;
use crate::model::{ModelController,Ticket,TicketForCreate};
use crate::prelude::*;

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}

async fn create_ticket(
    ctx: Ctx, // if Ctx unable to extract from cookie, this handler will not be fired
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>, 
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create ticket userid {:?}", "HANDLER", ctx.user_id());
    let ticket = mc.create_ticket(ticket_fc).await?;
    Ok(Json(ticket))
}

async fn list_tickets(
    State(mc): State<ModelController>,
) -> Result<Json<Vec<Ticket>>> {
    let res = mc.list_ticket().await?;
    Ok(Json(res))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    let res = mc.delete_ticket(id).await?;
    Ok(Json(res))
}