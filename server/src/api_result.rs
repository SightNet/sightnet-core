use salvo::{Depot, Request, Response, Writer};
use salvo::async_trait;
use salvo::writer::Json;
use serde::Serialize;
use serde_json::{json, Value};
use serde_json::Value::Null;

pub struct ApiResult {
    data: Option<Value>,
}

impl ApiResult {
    pub fn new(data: Option<Value>) -> Self {
        Self {
            data
        }
    }
}

#[async_trait]
impl Writer for ApiResult {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        if let Some(data) = self.data {
            res.render(Json(json!(data)))
        } else {
            res.render("")
        }
    }
}
