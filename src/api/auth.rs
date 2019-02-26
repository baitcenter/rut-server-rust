// api.auth view handler

use actix_web::{ 
    HttpResponse, HttpRequest, FutureResponse, AsyncResponder,
    FromRequest, Error, error, Json, State
};
use futures::Future;
use jwt::{decode, Validation};
use router::AppState;
use model::user::{ User, UserID, SignUser, LogUser, CheckUser, UpdateUser, Claims };

pub fn signup((sign_user, state): (Json<SignUser>, State<AppState>))
 -> FutureResponse<HttpResponse> {
    state.db.send(SignUser{
        uname: sign_user.uname.clone(),
        password: sign_user.password.clone(),
        confirm_password: sign_user.confirm_password.clone(),
    })
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into())
    })
    .responder()
}

pub fn check_user(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let uname = String::from(req.match_info().get("uname").unwrap());
    req.state().db.send(User::new("", &uname))
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into())
    })
    .responder()
}

pub fn signin((log_user, req): (Json<LogUser>, HttpRequest<AppState>))
 -> FutureResponse<HttpResponse> {
    req.state().db.send(LogUser{
        uname: log_user.uname.clone(),
        password: log_user.password.clone(),
    })
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into())
    })
    .responder()
}

// auth token
pub fn decode_token(token: &str) -> Result<CheckUser, Error> {
    let secret_key: String = dotenv::var("SECRET_KEY")
                    .expect("AHaRdGuESsSeCREkY");
    decode::<Claims>(token, secret_key.as_ref(), &Validation::default())
        .map(|data| Ok(data.claims.into()))
        .map_err(error::ErrorUnauthorized)?
}

impl<S> FromRequest<S> for CheckUser {
    type Config = ();
    type Result = Result<CheckUser, Error>;
    fn from_request(req: &HttpRequest<S>, _: &Self::Config) -> Self::Result {
        // println!("From Auth_Token: {:?}", req); 
        if let Some(auth_token) = req.headers().get("authorization") {
            if let Ok(i) = auth_token.to_str() {
               let user: CheckUser = decode_token(i)?;
               return Ok(user);
            }
        }
        Err(error::ErrorUnauthorized(401))
    }
}

pub fn auth_token(user: CheckUser) -> HttpResponse {
    HttpResponse::Ok().json(user)
}

pub fn get_user(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let userid = String::from(req.match_info().get("userid").unwrap());
    req.state().db.send( UserID{userid})
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into())
    })
    .responder()
}

pub fn update_user((user, req, auth): (Json<UpdateUser>, HttpRequest<AppState>, CheckUser))
 -> FutureResponse<HttpResponse> {
    req.state().db.send( UpdateUser{
        id: auth.id.clone(),
        uname: user.uname.clone(),
        avatar: user.avatar.clone(),
        email: user.email.clone(),
        intro: user.intro.clone(),
    })
    .from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(_) => Ok(HttpResponse::InternalServerError().into())
    })
    .responder()
}