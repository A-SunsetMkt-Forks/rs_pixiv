use crate::AppState;
use crate::download::{get_info, stream_file};
use crate::json_struct::Root;
use crate::tmplate::IndexTemp;
use actix_web::dev::Payload;
use actix_web::http;
use actix_web::{get, http::header::{ContentType,LAST_MODIFIED}, web, App, HttpResponse, HttpRequest, Responder};
use askama::Template;
use awc::http::header::{REFERER, USER_AGENT};
use awc::Client;
use log::error;
use serde::__private::de::Content;


#[get("/")]
pub async fn index() -> impl Responder {
    let rsp = IndexTemp {};
    return match rsp.render() {
        Ok(rsp) => HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(rsp),
        Err(_) => HttpResponse::InternalServerError().finish(),
    };
}

#[get("/json/{id}")]
pub async fn json_img(id: web::Path<i32>,data: web::Data<AppState>) -> impl Responder {
    let content = get_info(id.into_inner(),&data.client).await;
    match content {
        Some(mut i) => HttpResponse::Ok().content_type(ContentType::json()).body(i),
        None => HttpResponse::NotFound().finish(),
    }
}


#[get("/web/{id}")]
pub async fn web_img(id: web::Path<i32>,data: web::Data<AppState>,req : HttpRequest) ->  impl Responder {
    if  req.headers().contains_key("if-modified-since") {
       return HttpResponse::NotModified().finish();
    }
    let content = get_info(id.into_inner(),&data.client).await;
    match content {
        Some(i) => {
            let obj: Root = serde_json::from_str(&String::from_utf8(i.to_vec()).unwrap()).unwrap();
            let url =obj.body.urls.regular;
            stream_file(&url,&data.client).await
        }
     None=>{
            HttpResponse::NotFound().finish()
        }
     }  
 }



 #[get("/raw/{id}")]
 pub async fn raw_img(id: web::Path<i32>,data: web::Data<AppState>,req : HttpRequest) ->  impl Responder {
     if  req.headers().contains_key("if-modified-since") {
        return HttpResponse::NotModified().finish();
     }
     let content = get_info(id.into_inner(),&data.client).await;
     match content {
         Some(i) => {
             let obj: Root = serde_json::from_str(&String::from_utf8(i.to_vec()).unwrap()).unwrap();
             let url =obj.body.urls.original;
             stream_file(&url,&data.client).await
         }
      None=>{
             HttpResponse::NotFound().finish()
         }
      }  
  }
 
 
 