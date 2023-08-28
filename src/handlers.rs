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
