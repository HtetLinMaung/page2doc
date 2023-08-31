# Page2Doc Microservice

## Overview

`Page2Doc` is a microservice that converts web pages or HTML content to PDF documents. It provides a RESTful API to generate PDFs with various customization options like page format, margins, headers, footers, and more.

## Installation

### Docker Setup

To run the service using Docker, execute the following command:

```bash
docker run -e HOST_NAME=<host_name> -e API_KEY=<api_key> -e JWT_SECRET=<jwt_secret> -e DEFAULT_REQUEST_SIZE=<default_request_size> -p 8080:8080 htetlinmaung/page2doc
```

Environment Variables

- `HOST_NAME`: The hostname where the service is running.
- `API_KEY`: API key for authentication.
- `JWT_SECRET`: Secret for JWT authentication.
- `DEFAULT_REQUEST_SIZE`: (Optional) Default request size in bytes. Default is 2 MB.

### Docker Compose

You can also use Docker Compose to run the service. Create a `docker-compose.yml` file with the following content:

```yml
version: "3.9"

services:
  page2doc:
    image: htetlinmaung/page2doc
    ports:
      - "8080:8080"
    environment:
      - HOST_NAME=<host_name>
      - API_KEY=<api_key>
      - JWT_SECRET=<jwt_secret>
      - DEFAULT_REQUEST_SIZE=<default_request_size>
    volumes:
      - ./fonts:/usr/share/fonts/custom-fonts
```

Run the service using:

```bash
docker-compose up
```

#### Custom Fonts

To add custom fonts to the container, you can mount a volume that points to your local fonts directory. This will make the fonts available at `/usr/share/fonts/custom-fonts` inside the container.

## API Endpoints

- `POST /page2doc/create-report`: Generates a PDF based on the provided parameters.

## Request Parameters

| Parameter               | Type    | Description                                  | Default         |
| ----------------------- | ------- | -------------------------------------------- | --------------- |
| `html`                  | String  | HTML content to convert                      | None            |
| `css`                   | String  | CSS styles                                   | None            |
| `file_name`             | String  | Name of the output PDF file                  | None (Required) |
| `format`                | String  | Paper format ('A4', 'Letter', etc.)          | None            |
| `landscape`             | Boolean | Landscape mode                               | None            |
| `scale`                 | String  | Scale of the webpage rendering               | None            |
| `margin_top`            | String  | Top margin                                   | None            |
| `margin_bottom`         | String  | Bottom margin                                | None            |
| `margin_right`          | String  | Right margin                                 | None            |
| `margin_left`           | String  | Left margin                                  | None            |
| `header_template`       | String  | HTML template for the header                 | None            |
| `footer_template`       | String  | HTML template for the footer                 | None            |
| `display_header_footer` | Boolean | Display header and footer                    | None            |
| `prefer_css_page_size`  | Boolean | Prefer CSS page size over viewport size      | None            |
| `page_ranges`           | String  | Page ranges to print (e.g., '1-5, 8, 11-13') | None            |
| `ignore_http_errors`    | Boolean | Ignore HTTP errors during navigation         | None            |
| `wait_until`            | String  | When to consider navigation succeeded        | 'load'          |
| `timeout`               | String  | Maximum navigation time in milliseconds      | None            |
| `url`                   | String  | URL of the website to convert                | None            |

## Example Usage

To create a PDF from a URL:

```bash
curl -X POST http://localhost:8080/page2doc/create-report \
     -H "Content-Type: application/json" \
     -d '{"url": "https://example.com", "file_name": "example.pdf"}'
```

To create a PDF from HTML content:

```bash
curl -X POST http://localhost:8080/page2doc/create-report \
     -H "Content-Type: application/json" \
     -d '{"html": "<h1>Hello, World!</h1>", "file_name": "hello_world.pdf"}'
```

## Response

The service returns a JSON object with the following fields:

- `code`: HTTP status code
- `message`: Status message
- `url`: URL to download the generated PDF (if successful)

- `POST /generate-token`: Generates a token for secure PDF download.

## Request Parameters

| Parameter | Type   | Description                      | Required |
| --------- | ------ | -------------------------------- | -------- |
| `exp`     | Number | Token expiration time in seconds | Yes      |

## Example Usage

To generate a token, make a POST request to `/page2doc/generate-token` with the `exp` parameter specifying the token's expiration time in seconds.

```bash
curl -X POST http://localhost:8080/page2doc/generate-token \
     -H "x-api-key: <your_api_key>" \
     -H "Content-Type: application/json" \
     -d '{"exp": 3600}'
```

## Response

The service returns a JSON object with the following fields:

- `code`: HTTP status code
- `message`: Status message
- `token`: Generated token for secure PDF download

## Secure PDF Download

To download the PDF generated by `/page2doc/create-report`, you'll need a token generated by the `/page2doc/generate-token` API. Use the token as a query parameter like so:

```bash
<download-url>?t=<token>
```
