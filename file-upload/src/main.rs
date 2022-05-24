use bytes::BufMut;
use futures_util::TryStreamExt;
use handlebars::Handlebars;
use log::info;
use serde::Serialize;
use serde_json::json;
use std::{collections::HashMap, fs::File, io::prelude::*, path::Path, sync::Arc};
use warp::{
    http::{Response, Uri},
    multipart::{FormData, Part},
    Filter,
};

struct WithTemplate<T: Serialize> {
    name: &'static str,
    value: T,
}

fn render<T: Serialize>(
    template: WithTemplate<T>,
    hbs: Arc<Handlebars<'_>>,
) -> impl warp::Reply {
    let render = hbs
        .render(template.name, &template.value)
        .unwrap_or_else(|err| err.to_string());

    warp::reply::html(render)
}

async fn upload_file(form: FormData) -> Result<String, warp::Rejection> {
    let mut file_name = String::new();

    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("form error: {e}");
        warp::reject()
    })?;

    for p in parts {
        if p.name() == "uploaded" {
            file_name = p.filename().unwrap().to_string();

            let value = p
                .stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|e| {
                    eprintln!("error: {e}");
                    warp::reject()
                })?;

            tokio::fs::write(&file_name, value).await.map_err(|e| {
                eprintln!("error writing file: {e}");
                warp::reject()
            })?;
        }
    }

    Ok(file_name)
}

#[tokio::main]
async fn main() {
    let _ = pretty_env_logger::try_init();

    let template = include_str!("index.html");

    let mut hb = Handlebars::new();
    hb.register_template_string("template.html", template)
        .unwrap();

    let hb = Arc::new(hb);

    let handlebars = move |with_template| render(with_template, hb.clone());

    let get_file = warp::get()
        .and(warp::path("get_file"))
        .and(warp::query::<HashMap<String, String>>())
        .map(
            |query: HashMap<String, String>| match query.get("file_path") {
                Some(file_path) =>
                    if Path::new(&file_path).exists() {
                        let mut buffer = String::new();

                        if let Ok(mut file) = File::open(&file_path) {
                            if let Err(error) = file.read_to_string(&mut buffer) {
                                buffer.clear();
                                buffer = error.to_string();
                            }
                        }

                        if buffer.is_empty() {
                            Response::builder().body(format!(
                                "{file_path} either not able to read or empty"
                            ))
                        } else {
                            Response::builder().body(buffer)
                        }
                    } else {
                        Response::builder().body(String::from("file not found"))
                    },
                None => Response::builder()
                    .body(String::from("file_path query param not found")),
            },
        );

    let upload_file = warp::post()
        .and(warp::path("upload_file"))
        .and(warp::multipart::form().max_length(1_074_000_000)) // 1 GB
        .and_then(upload_file)
        .map(|file_path| {
            let uri: Uri = format!("/get_file?file_path={file_path}").parse().unwrap();

            warp::redirect(uri)
        });

    let index = warp::get()
        .and(warp::path::end())
        .map(|| WithTemplate {
            name: "template.html",
            value: json!({}),
        })
        .map(handlebars);

    let routes = index.or(get_file).or(upload_file);

    info!("Starting Warp server on http://0.0.0.0:3003...");
    warp::serve(routes).run(([0, 0, 0, 0], 3003)).await;
}
