use actix_web::{get, post, web, HttpResponse, Responder};
use std::{env, fs, process::Command};
use uuid::Uuid;

#[get("/")]
async fn index() -> impl Responder {
    "Page to Document server is online"
}

#[derive(serde::Deserialize, Debug)]
struct RequestBody {
    html: String,
    css: String,
    file_name: String,
}

#[derive(serde::Serialize)]
struct ResponseBody {
    code: u16,
    message: String,
    url: Option<String>,
}

#[post("/create-report")]
async fn create_files(body: web::Json<RequestBody>) -> impl Responder {
    println!("[create-report] body: {:#?}", body);
    let unique_folder_name = Uuid::new_v4().to_string();
    let path = format!("static/{}", unique_folder_name);

    if !fs::metadata(&path).is_ok() {
        if let Err(_) = fs::create_dir_all(&path) {
            return HttpResponse::InternalServerError().json(ResponseBody {
                code: 500,
                message: "Internal Server Error".into(),
                url: None,
            });
        }
    }

    let html_content = format!(
        "<!DOCTYPE html>\n<html>\n<head>\n<link rel=\"stylesheet\" type=\"text/css\" href=\"style.css\">\n</head>\n<body>\n{}\n</body>\n</html>",
        body.html
    );

    let html_path = format!("{}/index.html", &path);
    if let Err(_) = fs::write(&html_path, html_content) {
        return HttpResponse::InternalServerError().json(ResponseBody {
            code: 500,
            message: "Internal Server Error".into(),
            url: None,
        });
    }

    let css_path = format!("{}/style.css", &path);
    if let Err(_) = fs::write(&css_path, &body.css) {
        return HttpResponse::InternalServerError().json(ResponseBody {
            code: 500,
            message: "Internal Server Error".into(),
            url: None,
        });
    }

    let host_name = env::var("host_name").unwrap_or("http://localhost:8080".to_string());
    let url = format!(
        "{}/page2doc/static/{}/index.html",
        host_name, unique_folder_name
    );

    let output_file = format!("{}/{}", &path, &body.file_name);
    let command = Command::new("sitetopdf")
        .arg("-u")
        .arg(&url)
        .arg("-o")
        .arg(output_file)
        .output();

    match command {
        Ok(output) => {
            if !output.status.success() {
                return HttpResponse::InternalServerError().json(ResponseBody {
                    code: 500,
                    message: "Failed to run sitetopdf command".into(),
                    url: None,
                });
            }
            if let Err(_) = fs::remove_file(&html_path) {
                return HttpResponse::InternalServerError().json(ResponseBody {
                    code: 500,
                    message: "Failed to delete HTML file".into(),
                    url: None,
                });
            }

            if let Err(_) = fs::remove_file(&css_path) {
                return HttpResponse::InternalServerError().json(ResponseBody {
                    code: 500,
                    message: "Failed to delete CSS file".into(),
                    url: None,
                });
            }
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(ResponseBody {
                code: 500,
                message: "Error executing sitetopdf command".into(),
                url: None,
            });
        }
    }

    HttpResponse::Ok().json(ResponseBody {
        code: 200,
        message: "Report created successfully".into(),
        url: Some(format!(
            "{}/page2doc/static/{}/{}",
            host_name, unique_folder_name, body.file_name
        )),
    })
}
