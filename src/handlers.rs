use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tide::{Request, Response, StatusCode};
use crate::db_controller::Database;

#[derive(Serialize, Deserialize)]
struct EntryInput {
    name: String,
    value: i32,
}

pub async fn add_entry(mut req: Request<Arc<Database>>) -> tide::Result {
    let db = req.state().clone();
    let EntryInput { name, value } = req.body_json().await?;
    db.add_entry(&name, value)?;
    Ok(Response::new(StatusCode::Created))
}

pub async fn get_entries(mut req: Request<Arc<Database>>) -> tide::Result {
    let db = req.state().clone();
    let entries = db.get_all_entries()?;
    let response = Response::builder(StatusCode::Ok)
        .body(serde_json::to_string(&entries)?)
        .build();
    Ok(response)
}

pub async fn get_entry(mut req: Request<Arc<Database>>) -> tide::Result {
    let db = req.state().clone();
    let name = req.param("name")?;
    match db.get_entry(name)? {
        Some(entry) => Ok(Response::new(StatusCode::Ok).body_json(&entry)?),
        None => Ok(Response::new(StatusCode::NotFound)),
    }
}

pub async fn delete_entry(mut req: Request<Arc<Database>>) -> tide::Result {
    let db = req.state().clone();
    let name = req.param("name")?;
    let num_deleted = db.remove_entry(name)?;
    if num_deleted > 0 {
        Ok(Response::new(StatusCode::Ok))
    } else {
        Ok(Response::new(StatusCode::NotFound))
    }
}

pub async fn update_entry(mut req: Request<Arc<Database>>) -> tide::Result {
    let db = req.state().clone();
    let name = req.param("name")?;
    let EntryInput { name: new_name, value } = req.body_json().await?;
    if name != new_name {
        return Ok(Response::new(StatusCode::BadRequest).body_string("Name in URL does not match"))
    }
    db.add_entry(&new_name, value)?;
    Ok(Response::new(StatusCode::Ok))
}