use crate::state::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use crate::dao::{{table}}::{ {{Table}}, {{Table}}Request };
use crate::api::ApiResult;


#[get("/{{table}}s")]
async fn query(params: web::Query<Params>,state: AppState) -> impl Responder {
    let result = {{Table}}::query( params.into_inner(),state).await;
    match result {
        Ok({{table}}) => {
            ApiResult::new().with_data({{table}})
        }
        Err(e) => {
            ApiResult::new().code(200).with_msg(e.to_string())
        }
    }
}


#[get("/{{table}}/{id}")]
async fn find_by_id(id: web::Path<i32>, state: AppState) -> impl Responder {
    let result = {{Table}}::find_by_id(id.into_inner(), state).await;
    match result {
        Ok({{table}}) => {
            ApiResult::new().with_data({{table}})
        }
        Err(e) => {
            ApiResult::new().code(200).with_msg(e.to_string())
        }
    }
}

#[post("/{{table}}")]
async fn create({{table}}: web::Json<TodoRequest>, state: AppState) -> impl Responder {
    let result = {{Table}}::create({{table}}.into_inner(), state).await;
    match result {
        Ok({{table}}) => {
            ApiResult::new().with_data({{table}})
        }
        Err(e) => {
            ApiResult::new().code(200).with_msg(e.to_string())
        }
    }
}

#[put("/{{table}}/{id}")]
async fn update(id: web::Path<i32>, {{table}}: web::Json<{{Table}}Request>, state: AppState) -> impl Responder {
    let result = {{Table}}::update(id.into_inner(), {{table}}.into_inner(), state).await;
    match result {
        Ok({{table}}) => {
            ApiResult::new().with_data({{table}})
        }
        Err(e) => {
            ApiResult::new().code(200).with_msg(e.to_string())
        }
    }

}

#[delete("/{{table}}/{id}")]
async fn delete(id: web::Path<i32>, state: AppState) -> impl Responder {
    let result = {{Table}}::delete(id.into_inner(), state).await;
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


