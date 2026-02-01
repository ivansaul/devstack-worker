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
        .get_async("/api/cheatsheets", cheatsheets::handlers::list)
        .get_async("/api/cheatsheets/:id", cheatsheets::handlers::get)
        .run(req, env)
        .await
}
