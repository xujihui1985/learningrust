use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestPartsExt;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

use crate::ctx::Ctx;
use crate::prelude::{Error, Result};
use crate::web::AUTH_TOKEN;

pub async fn mw_require_auth<B>(
    ctx: Result<Ctx>, // can be Option<Ctx> or Ctx
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:>12} - middleware", "MW_REQUIRE_AUTH");
    ctx?;
    // let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    // let Some(token) = auth_token else {
    //     return Err(Error::AuthFailNoAuthToken);
    // };

    // let (user_id, exp, sign) = auth_token
    //     .ok_or(Error::AuthFailNoAuthToken)
    //     .and_then(parse_token)?;

    // valid token

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver<B>(
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthToken)
        .and_then(parse_token)
    {
        Ok((user_id, _, _)) => Ok(Ctx::new(user_id)),
        Err(e) => Err(e),
    };
    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthToken)) {
        cookies.remove(Cookie::named(AUTH_TOKEN));
    }

    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut axum::http::request::Parts, state: &S) -> Result<Self> {
        println!("->> {:>12} - Ctx", "EXTRACTOR");
        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInRequest)?
            .clone()
        // let cookies = parts.extract::<Cookies>().await.unwrap();

        // let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        // let (user_id, _, _) = auth_token
        //     .ok_or(Error::AuthFailNoAuthToken)
        //     .and_then(parse_token)?;

        // Ok(Ctx::new(user_id))
    }
}

fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
        .ok_or(Error::AuthFailTokenWrongFormat)?;
    let user_id = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;
    Ok((user_id, exp.to_string(), sign.to_string()))
}
