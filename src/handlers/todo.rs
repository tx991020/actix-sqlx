
use crate::state::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use crate::dao::todo::{Todo, TodoRequest,TodoFilter};
use crate::api::ApiResult;


#[get("/todos")]
async fn query(params: web::Query<TodoFilter>,state: AppState) -> impl Responder {
    let result = Todo::query( params.into_inner(),state).await;
    match result {
        Ok(todo) => {
            ApiResult::new().with_data(todo)
        }
        Err(e) => {
            ApiResult::new().code(200).with_msg(e.to_string())
        }
    }
}


#[get("/todo/{id}")]
async fn find_by_id(id: web::Path<i32>, state: AppState) -> impl Responder {
    let result = Todo::find_by_id(id.into_inner(), state).await;
    match result {
        Ok(todo) => {
            ApiResult::new().with_data(todo)
        }
        Err(e) => {
            ApiResult::new().code(200).with_msg(e.to_string())
        }
    }
}

#[post("/todo")]
async fn create(todo: web::Json<TodoRequest>, state: AppState) -> impl Responder {
    let result = Todo::create(todo.into_inner(), state).await;
    match result {
        Ok(todo) => {
            ApiResult::new().with_data(todo)
        }
        Err(e) => {
            ApiResult::new().code(200).with_msg(e.to_string())
        }
    }
}

#[put("/todo/{id}")]
async fn update(id: web::Path<i32>, todo: web::Json<TodoRequest>, state: AppState) -> impl Responder {
    let result = Todo::update(id.into_inner(), todo.into_inner(), state).await;
    match result {
        Ok(todo) => {
            ApiResult::new().with_data(todo)
        }
        Err(e) => {
            ApiResult::new().code(200).with_msg(e.to_string())
        }
    }

}

#[delete("/todo/{id}")]
async fn delete(id: web::Path<i32>, state: AppState) -> impl Responder {
    let result = Todo::delete(id.into_inner(), state).await;
    match result {
        Ok(rows) => {
            if rows > 0 {
                ApiResult::new().code(200).with_data(rows)
            } else {
                ApiResult::new().code(200).with_msg("not found")
            }
        }
        Err(e) => {
            ApiResult::new().code(200).with_msg(e.to_string())
        }
    }
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(query);
    cfg.service(find_by_id);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}

