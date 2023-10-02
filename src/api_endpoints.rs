use crate::shared_ops::SharedCredAndOps;
use actix_web::{get, post, web, HttpResponse, Responder};
use std::fs::create_dir;

#[post("/create_bucket")]
async fn create_bucket(
    paylod: web::Json<String>,
    shared: web::Data<SharedCredAndOps>,
) -> impl Responder {
    let bucket_name = paylod.into_inner();
    shared.s3_ops().create_bucket(&bucket_name).await;
    match bucket_name.is_empty() {
        false => HttpResponse::Ok().body("Success"),
        true => HttpResponse::BadRequest().body("Received Empty string Value"),
    }
}
#[get("/get_buckets")]
async fn get_bucket_lists(shared: web::Data<SharedCredAndOps>) -> impl Responder {
    let get_bucket_lists = shared.s3_ops().get_buckets().await;
    match get_bucket_lists.is_empty() {
        false => HttpResponse::Ok().json(get_bucket_lists),
        true => HttpResponse::NoContent().json("No Buckets to Display"),
    }
}

#[get("/get_bucket_keys/{bucket_name}")]
async fn get_bucket_keys(
    bucket_name: web::Path<String>,
    shared: web::Data<SharedCredAndOps>,
) -> impl Responder {
    let bucket_name = bucket_name.into_inner();
    let keys = shared
        .s3_ops()
        .list_objects_given_prefix(&bucket_name, "")
        .await;
    match keys.is_empty() {
        false => HttpResponse::Ok().json(keys),
        true => HttpResponse::NoContent().json("No keys to Return"),
    }
}
#[post("/generate_polly_audios/{engine_name}/{language_code}")]
async fn generate_polly_audios(
    param: web::Path<(String, String)>,
    ssml_text: web::Json<String>,
    shared: web::Data<SharedCredAndOps>,
) -> impl Responder {
    let (engine_name, language_code) = param.into_inner();
    let polly_ops = shared.polly_ops();
    match (
        engine_name.is_empty(),
        language_code.is_empty(),
        ssml_text.is_empty(),
    ) {
        (false, false, false) => {
            let path_prefix = format!("{engine_name}");
            create_dir(&path_prefix).expect("Error while creating path prefix for polly audios");
            //let speech_text =std::fs::read_to_string(ssml_text_path).unwrap();
            polly_ops
                .generate_all_available_voices_in_mp3(
                    &ssml_text,
                    &language_code,
                    &engine_name,
                    &path_prefix,
                )
                .await;
            HttpResponse::Ok()
                .body("All voices are generated in the directory where the server is running")
        }
        _ => HttpResponse::BadRequest().body("The parameters are empty"),
    }
}
#[post("/get_text_given_image_stored_on_bucket")]
async fn get_text(
    payload: web::Json<(String, String)>,
    shared: web::Data<SharedCredAndOps>,
) -> impl Responder {
    let (bucket_name, key_image_name) = payload.into_inner();
    let rekognition_ops = shared.rekognition_ops();
    let text_detail = rekognition_ops
        .detect_texts(&bucket_name, &key_image_name)
        .await;
    let mut texts = String::new();
    text_detail.into_iter().for_each(|mut detail| {
        let text = detail.detected_text();
        if let Some(texts_) = text {
            texts.push_str(&texts_);
            texts.push_str("  ");
        }
    });
    HttpResponse::Ok().body(texts)
}
pub fn configure_path(config: &mut web::ServiceConfig) {
    config
        .service(create_bucket)
        .service(get_text)
        .service(get_bucket_lists)
        .service(get_bucket_keys)
        .service(generate_polly_audios);
}
