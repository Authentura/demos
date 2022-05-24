use bytes::BufMut;
use futures_util::TryStreamExt;
use handlebars::Handlebars;
use log::info;
use serde::Serialize;
use serde_json::json;
use std::{
    collections::HashMap,
    fs::{read_dir, File},
    io::prelude::*,
    path::Path,
    sync::Arc,
};
use warp::{
    http::{Response, Uri},
    multipart::{FormData, Part},
    Filter,
};

struct WithTemplate<T: Serialize> {
    name: &'static str,
    value: T,
}

#[derive(Serialize, Default, Clone)]
struct Entry {
    name: String,
    path: String,
    content_type: String,
    is_dir: bool,
    is_file: bool,
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

    let base_path = option_env!("BASE_PATH");

    let base_path = if let Some(base_path) = base_path {
        base_path
    } else {
        "/"
    };

    let index = include_str!("../templates/index.html");
    let file_index = include_str!("../templates/file_index.html");
    let error = include_str!("../templates/error.html");

    let mut hb = Handlebars::new();
    hb.register_template_string("index.html", index).unwrap();
    hb.register_template_string("file_index.html", file_index)
        .unwrap();
    hb.register_template_string("error.html", error).unwrap();

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

    let bp = base_path.to_owned();
    let file_listing = warp::get()
        .and(warp::path("files"))
        .and(warp::query::<HashMap<String, String>>())
        .map(move |query: HashMap<String, String>| match query.get("path") {
            // Some(path) => if let Ok(directory) = read_dir(path) {} else {Response},
            Some(path) => match read_dir(path) {
                Ok(dir_contents) => {
                    let mut bp = bp.clone();

                    if bp == "/" {
                        bp = String::from("");
                    }

                    let mut directory_listing = vec![
                        Entry {
                            name: String::from("."),
                            path: Path::new(&format!("{path}/.")).canonicalize().unwrap().as_os_str().to_string_lossy().into_owned(),
                            content_type: String::from("Directory"),
                            is_dir: true,
                            is_file: false,
                        },
                        Entry {
                            name: String::from(".."),
                            path: Path::new(&format!("{path}/..")).canonicalize().unwrap().as_os_str().to_string_lossy().into_owned(),
                            content_type: String::from("Directory"),
                            is_dir: true,
                            is_file: false,
                        },
                    ];

                    for dir_entry in dir_contents.flatten() {
                        let mut entry = Entry {
                            ..Default::default()
                        };

                        entry.name = dir_entry.file_name().as_os_str().to_string_lossy().into_owned();

                        entry.path = match dir_entry
                            .path()
                            .canonicalize()
                            {
                                Ok(p) => p,
                                Err(_) => continue,
                            }
                            .as_os_str()
                            .to_string_lossy()
                            .into_owned();

                        entry.content_type = if let Ok(file_type) = dir_entry.file_type()
                        {
                            if file_type.is_dir() {
                                entry.is_dir = true;
                                String::from("Directory")
                            } else if file_type.is_file() {
                                entry.is_file = true;
                                String::from("File")
                            } else if file_type.is_symlink() {
                                String::from("Symbolic Link")
                            } else {
                                String::from("Unknown")
                            }
                        } else {
                            String::from("Unknown")
                        };

                        directory_listing.push(entry);
                    }

                    WithTemplate {
                        name: "file_index.html",
                        value: json!({ "base_path": bp, "directory": path, "directory_contents": directory_listing }),
                    }
                },
                Err(error) => WithTemplate {
                    name: "error.html",
                    value: json!(error.to_string()),
                },
            },
            None => WithTemplate {
                name: "error.html",
                value: json!("path query param not found"),
            },
        })
        .map(handlebars.clone());

    let bp = base_path.to_owned();
    let upload_file = warp::post()
        .and(warp::path("upload_file"))
        .and(warp::multipart::form().max_length(1_074_000_000)) // 1 GB
        .and_then(upload_file)
        .map(move |file_path| {
            let mut bp = bp.clone();

            if bp == "/" {
                bp = String::from("");
            }

            let uri: Uri = if !bp.is_empty() {
                format!("/{bp}/get_file?file_path={file_path}").parse().unwrap()
            } else {
                format!("/get_file?file_path={file_path}").parse().unwrap()
            };

            info!("{:?}", &uri);

            warp::redirect(uri)
        });

    let bp = base_path.to_owned();
    let index = warp::get()
        .and(warp::path::end())
        .map(move || {
            let mut bp = bp.clone();

            if bp == "/" {
                bp = String::from("");
            }

            WithTemplate {
                name: "index.html",
                value: json!({ "base_path": bp }),
            }
        })
        .map(handlebars);

    let subroutes = get_file.or(upload_file).or(file_listing);

    info!("BASE_PATH={base_path}");
    info!("Starting Warp server on http://0.0.0.0:3003...");

    if base_path != "/" && !base_path.is_empty() {
        warp::serve(index.or(warp::path(base_path).and(subroutes)))
            .run(([0, 0, 0, 0], 3003))
            .await;
    } else {
        warp::serve(index.or(subroutes))
            .run(([0, 0, 0, 0], 3003))
            .await;
    }
}
