pub(crate) mod cheatsheets;
pub(crate) mod error;

use crate::cheatsheets::repository::CheatsheetRepository;
use worker::*;

struct AppState {
    cheatsheets: CheatsheetRepository,
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let cheatsheets_db = env.d1("CHEATSHEETS")?;
    let router = Router::with_data(AppState {
        cheatsheets: CheatsheetRepository::new(cheatsheets_db),
    });
    router
        .get_async("/cheatsheets", |_req, ctx| async {
            let repo = ctx.data.cheatsheets;
            match repo.list_meta().await {
                Ok(items) => Response::from_json(&items),
                Err(err) => err.into(),
            }
        })
        .get_async("/cheatsheets/:id", |_req, ctx| async move {
            let Some(id) = ctx.param("id") else {
                return Response::error("Missing ID parameter", 400);
            };
            let repo = &ctx.data.cheatsheets;
            match repo.fetch_by_id(id).await {
                Ok(item) => Response::from_json(&item),
                Err(err) => err.into(),
            }
        })
        .run(req, env)
        .await
}
