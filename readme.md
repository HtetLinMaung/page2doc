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
