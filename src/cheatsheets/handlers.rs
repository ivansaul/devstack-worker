use crate::AppState;
use worker::{Request, Response, Result, RouteContext};

pub async fn list(_req: Request, ctx: RouteContext<AppState>) -> Result<Response> {
    let repo = &ctx.data.cheatsheets;
    match repo.list_meta().await {
        Ok(items) => Response::from_json(&items),
        Err(err) => err.into(),
    }
}

pub async fn get(_req: Request, ctx: RouteContext<AppState>) -> Result<Response> {
    let Some(id) = ctx.param("id") else {
        return Response::error("Missing ID parameter", 400);
    };
    let repo = &ctx.data.cheatsheets;
    match repo.get(id).await {
        Ok(item) => Response::from_json(&item),
        Err(err) => err.into(),
    }
}
