use std::{
    collections::HashMap,
    path::Path,
    process::Command,
    sync::{Arc, Mutex},
};

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

struct State {
    currently_updating: Arc<Mutex<HashMap<String, bool>>>,
}

async fn webhook(path: web::Path<(String, String)>, state: web::Data<State>) -> impl Responder {
    let (project, service) = path.into_inner();
    let path = Path::new("/compose/").join(&project);

    if let Some(prev_value) = state
        .currently_updating
        .lock()
        .unwrap()
        .insert(format!("{}/{}", &project, &service), true)
    {
        if prev_value {
            eprintln!("Already updating {}/{}", &project, &service);
            return HttpResponse::Conflict();
        }
    }

    let set_updating_false = || {
        state
            .currently_updating
            .lock()
            .unwrap()
            .remove(&format!("{}/{}", &project, &service));
    };
    println!("Received update request for {}/{}", &project, &service);

    let container_id = match Command::new("docker")
        .args(["compose", "ps", "-q", &service])
        .current_dir(&path)
        .output()
    {
        Ok(output) => {
            if output.stderr.is_empty() {
                String::from_utf8(output.stdout)
                    .expect("output of docker compose ps -q <service> is not a valid String")
                    .trim()
                    .to_string()
            } else {
                eprintln!("service not found");
                set_updating_false();
                return HttpResponse::NotFound();
            }
        }
        Err(_) => {
            eprintln!("project not found");
            set_updating_false();
            return HttpResponse::NotFound();
        }
    };

    let label_webhooks_update = match Command::new("docker")
        .args([
            "inspect",
            "--format",
            "'{{ index .Config.Labels \"composehook.update\"}}'",
            &container_id,
        ])
        .output()
    {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim() == "'true'",
        Err(e) => {
            eprintln!("{:#?}", e);
            set_updating_false();
            return HttpResponse::InternalServerError();
        }
    };

    if !label_webhooks_update {
        eprintln!("label composehook.update not set to true");
        set_updating_false();
        return HttpResponse::BadRequest();
    }

    match Command::new("docker")
        .args(["compose", "pull", &service])
        .current_dir(&path)
        .status()
    {
        Ok(_) => {
            match Command::new("docker")
                .arg("compose")
                .arg("up")
                .arg("-d")
                .arg(&service)
                .current_dir(&path)
                .spawn()
            {
                Ok(_) => {
                    println!("successfully updated!");
                    set_updating_false();
                    HttpResponse::Ok()
                }
                Err(e) => {
                    eprintln!("{:#?}", e);
                    set_updating_false();
                    HttpResponse::InternalServerError()
                }
            }
        }
        Err(e) => {
            eprintln!("{:#?}", e);
            set_updating_false();
            HttpResponse::InternalServerError()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let currently_updating = Arc::new(Mutex::new(HashMap::<String, bool>::new()));

    HttpServer::new(move || {
        App::new()
            .route("/{project}/{container}", web::get().to(webhook))
            .route("/{project}/{container}", web::post().to(webhook))
            .app_data(web::Data::new(State {
                currently_updating: currently_updating.clone(),
            }))
    })
    .bind(("0.0.0.0", 8537))?
    .run()
    .await
}
