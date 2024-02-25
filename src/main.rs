use std::{path::Path, process::Command};

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/{project}/{service}")]
async fn webhook(path: web::Path<(String, String)>) -> impl Responder {
    let (project, service) = path.into_inner();
    let path = Path::new("/compose/").join(&project);

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
                return HttpResponse::NotFound();
            }
        }
        Err(_) => {
            eprintln!("project not found");
            return HttpResponse::NotFound();
        }
    };

    let label_webhooks_update = match Command::new("docker")
        .args([
            "inspect",
            "--format",
            "'{{ index .Config.Labels \"webhooks.update\"}}'",
            &container_id,
        ])
        .output()
    {
        Ok(output) => String::from_utf8_lossy(&output.stdout) == "true",
        Err(e) => {
            eprintln!("{:#?}", e);
            return HttpResponse::InternalServerError();
        }
    };

    if !label_webhooks_update {
        eprintln!("label webhooks.update not set to true");
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
                    HttpResponse::Ok()
                }
                Err(e) => {
                    eprintln!("{:#?}", e);
                    HttpResponse::InternalServerError()
                }
            }
        }
        Err(e) => {
            eprintln!("{:#?}", e);
            HttpResponse::InternalServerError()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(webhook))
        .bind(("0.0.0.0", 9411))?
        .run()
        .await
}