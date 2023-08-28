use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

use crate::utils::{sign_token, verify_token, Claims};

#[get("/")]
async fn index() -> impl Responder {
    "Page to Document server is online"
}

#[derive(Deserialize, Debug)]
struct RequestBody {
    html: String,
    css: String,
    file_name: String,
    format: Option<String>,
    landscape: Option<bool>,
    scale: Option<String>,
    margin_top: Option<String>,
    margin_bottom: Option<String>,
    margin_right: Option<String>,
    margin_left: Option<String>,
    header_template: Option<String>,
    footer_template: Option<String>,
    display_header_footer: Option<bool>,
    prefer_css_page_size: Option<bool>,
    page_ranges: Option<String>,
    ignore_http_errors: Option<bool>,
    wait_until: Option<String>,
    timeout: Option<String>,
}

#[derive(Serialize)]
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

    let pdf_folder_path = format!("pdf/{unique_folder_name}");
    if !fs::metadata(&pdf_folder_path).is_ok() {
        if let Err(_) = fs::create_dir_all(&pdf_folder_path) {
            return HttpResponse::InternalServerError().json(ResponseBody {
                code: 500,
                message: "Internal Server Error".into(),
                url: None,
            });
        }
    }

    let output_file = format!("{}/{}", &pdf_folder_path, &body.file_name);
    let mut sitetopdf = Command::new("sitetopdf");
    sitetopdf.arg("-u").arg(&url).arg("-o").arg(output_file);

    if let Some(format) = &body.format {
        sitetopdf.arg("--format").arg(format);
    }
    if let Some(landscape) = body.landscape {
        if landscape {
            sitetopdf.arg("--landscape");
        }
    }
    if let Some(scale) = &body.scale {
        sitetopdf.arg("--scale").arg(scale);
    }
    if let Some(margin_top) = &body.margin_top {
        sitetopdf.arg("--margin-top").arg(margin_top);
    }
    if let Some(margin_bottom) = &body.margin_bottom {
        sitetopdf.arg("--margin-bottom").arg(margin_bottom);
    }
    if let Some(margin_right) = &body.margin_right {
        sitetopdf.arg("--margin-right").arg(margin_right);
    }
    if let Some(margin_left) = &body.margin_left {
        sitetopdf.arg("--margin-left").arg(margin_left);
    }
    if let Some(header_template) = &body.header_template {
        sitetopdf.arg("--header-template").arg(header_template);
    }
    if let Some(footer_template) = &body.footer_template {
        sitetopdf.arg("--footer-template").arg(footer_template);
    }
    if let Some(display_header_footer) = body.display_header_footer {
        if display_header_footer {
            sitetopdf.arg("--display-header-footer");
        }
    }
    if let Some(prefer_css_page_size) = body.prefer_css_page_size {
        if prefer_css_page_size {
            sitetopdf.arg("--prefer-css-page-size");
        }
    }
    if let Some(page_ranges) = &body.page_ranges {
        sitetopdf.arg("--page-ranges").arg(page_ranges);
    }
    if let Some(ignore_http_errors) = body.ignore_http_errors {
        if ignore_http_errors {
            sitetopdf.arg("--ignore-http-errors");
        }
    }
    if let Some(wait_until) = &body.wait_until {
        sitetopdf.arg("--wait-until").arg(wait_until);
    }
    if let Some(timeout) = &body.timeout {
        sitetopdf.arg("--timeout").arg(timeout);
    }

    let command = sitetopdf.output();

    match command {
        Ok(output) => {
            if !output.status.success() {
                return HttpResponse::InternalServerError().json(ResponseBody {
                    code: 500,
                    message: "Failed to run sitetopdf command".into(),
                    url: None,
                });
            }
            if let Err(_) = fs::remove_dir_all(&path) {
                return HttpResponse::InternalServerError().json(ResponseBody {
                    code: 500,
                    message: "Failed to delete folder".into(),
                    url: None,
                });
            }
            // if let Err(_) = fs::remove_file(&html_path) {
            //     return HttpResponse::InternalServerError().json(ResponseBody {
            //         code: 500,
            //         message: "Failed to delete HTML file".into(),
            //         url: None,
            //     });
            // }
            // if let Err(_) = fs::remove_file(&css_path) {
            //     return HttpResponse::InternalServerError().json(ResponseBody {
            //         code: 500,
            //         message: "Failed to delete CSS file".into(),
            //         url: None,
            //     });
            // }
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
            "{}/page2doc/pdf/{}/{}",
            host_name, unique_folder_name, body.file_name
        )),
    })
}

#[derive(Deserialize)]
struct TokenRequest {
    exp: usize,
}

#[derive(Serialize)]
struct TokenResponse {
    code: u16,
    message: String,
    token: Option<String>,
}

#[post("/generate-token")]
async fn generate_token(
    req: HttpRequest,
    body: web::Json<TokenRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let api_key = env::var("API_KEY").expect("API_KEY must be set");

    if let Some(header_value) = req.headers().get("x-api-key") {
        if header_value.to_str().unwrap() != api_key {
            return Ok(HttpResponse::Unauthorized().json(TokenResponse {
                code: 401,
                message: "Unauthorized".to_string(),
                token: None,
            }));
        }
    } else {
        return Ok(HttpResponse::Unauthorized().json(TokenResponse {
            code: 401,
            message: "Unauthorized".to_string(),
            token: None,
        }));
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;
    let claims = Claims {
        exp: now + body.exp,
    };
    let token = match sign_token(&claims) {
        Ok(t) => t,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(TokenResponse {
                code: 500,
                message: "Failed to generate token".to_string(),
                token: None,
            }));
        }
    };

    Ok(HttpResponse::Ok().json(TokenResponse {
        code: 200,
        message: "Token generated successfully".to_string(),
        token: Some(token),
    }))
}

#[get("/pdf/{uuid}/{filename}")]
async fn get_pdf(
    req: HttpRequest,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    // Extract JWT token from query parameter
    let token: Option<String> = req.query_string().split('=').nth(1).map(|s| s.to_string());

    // Verify JWT token
    if token.is_none() || !verify_token(&token.unwrap()) {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    // Check if the file exists
    let file_path = format!("./pdf/{}/{}", path.0, path.1);
    if !PathBuf::from(&file_path).exists() {
        return Ok(HttpResponse::NotFound().finish());
    }

    // Serve the PDF file
    Ok(HttpResponse::Ok().body(actix_web::web::Bytes::from(std::fs::read(file_path)?)))
}
