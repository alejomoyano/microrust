#[warn(dead_code)]
use actix_web::{get, post, web::Path, Responder, web::Json,HttpResponse,HttpServer,App};
use dotenv::{from_filename};
use std::env;
use mongodb::{bson::doc, options::ClientOptions, Client, Database,Cursor};

mod models;

fn get_env() -> models::Config {
    from_filename(".env").expect("Failed to read configuration .env file");
    models::Config {
        host : env::var("HOST").expect("Failed to read host"),
        port : env::var("PORT").expect("Failed to read port").parse::<u16>().unwrap(),
        db_name : env::var("DB_NAME").expect("Failed to read db name"),
        db_user : env::var("DB_USER").expect("Failed to read db user"),
        db_pass : env::var("DB_PASS").expect("Failed to read db pass"),
        db_uri : env::var("DB_URI").expect("Failed to read db uri"),
    }
}

async fn connect_to_mongodb() -> Database{
    let config: models::Config = get_env();

    let client_options = ClientOptions::parse(config.db_uri)
    .await
    .unwrap(); 
    let client = Client::with_options(client_options).unwrap();
    client.database(&config.db_name)
}

#[get("/user/{user_id}")]
async fn get_document(document: Path<models::DocumentIdentifier>) -> impl Responder {
    let connection = connect_to_mongodb().await;
    let cursor : Cursor<models::Document> = connection
        .collection("documents")
        .find(doc! {"name": &document.document_id}, None) //TODO -> cambiar
        .await
        .unwrap();
    let result = cursor.current();
    HttpResponse::Ok().json(result)
}

#[post("/user/insert")]
async fn insert_document(document_data: Json<models::Document>) -> impl Responder {
    let connection = connect_to_mongodb().await;
    let result = connection
        .collection("documents")
        .insert_one(document_data, None)
        .await
        .unwrap();
    HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error>{
    let config = get_env();

    HttpServer::new(|| {
        App::new()
        .service(get_document)
        .service(insert_document)
    })
    .bind((config.host, config.port))?
    .run()
    .await  
}
