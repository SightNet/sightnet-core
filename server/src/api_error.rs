use salvo::{Depot, Request, Response, Writer};
use salvo::async_trait;
use salvo::hyper::StatusCode;
use salvo::writer::Json;
use serde_json::json;

pub struct ApiError {
    code: i32,
    msg: &'static str,
}

impl ApiError {
    pub fn new(code: i32, msg: &'static str) -> Self {
        Self {
            code,
            msg,
        }
    }
}

#[async_trait]
impl Writer for ApiError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.render(Json(json!({
            "error": {
                "code": self.code,
                "detail": self.msg
            }
        })));

        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
    }
}