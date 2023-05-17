use rocket::{
    request::{self, FromRequest, Request, Outcome},
    http::Status,
};
use base64::{Engine as _, engine::general_purpose};

pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    fn from_authorization_header(header: &str) -> Option<BasicAuth> {
        let split = header.split_whitespace().collect::<Vec<_>>();
        if split.len() != 2 {
            return None;
        }

        if split[0] != "Basic" {
            return None;
        }
        
        Self::from_base64(split[1])
        
    }

    fn from_base64(encoded: &str) -> Option<BasicAuth> {
        let decoded = general_purpose::URL_SAFE.decode(encoded).ok()?;
        let decoded = String::from_utf8(decoded).ok()?;
        let split = decoded.split(":").collect::<Vec<_>>();
        if split.len() != 2 {
            return None;
        }

        Some(BasicAuth {
            username: split[0].to_string(),
            password: split[1].to_string(),
        })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header) = auth_header {
            if let Some(basic_auth) = Self::from_authorization_header(auth_header) {
                return Outcome::Success(basic_auth);
            }
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}
