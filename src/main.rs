mod db;

use tide::{Body, Request, Response, StatusCode};

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/new_user/:name/:password").get(new_user);
    app.at("/verify_user/:name/:password").get(verify_user);

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

async fn new_user(req: Request<()>) -> tide::Result {
    let name = req.param("name");
    let password = req.param("password");

    return match (name, password) {
        (Ok(name), Ok(password)) => {
            db::new_user(name, password).await?;
            Ok(Response::new(StatusCode::Ok))
        }
        _ => Ok(Response::new(StatusCode::BadRequest)),
    };
}

async fn verify_user(req: Request<()>) -> tide::Result {
    let name = req.param("name");
    let password = req.param("password");

    return match (name, password) {
        (Ok(name), Ok(password)) => {
            let verified = db::verify_user(name, password).await?;
            
            let mut res = match verified {
                true => Response::new(StatusCode::Ok),
                false => Response::new(StatusCode::Unauthorized),
            };

            res.set_body(Body::from_json(&verified)?);

            Ok(res)
        }
        _ => Ok(Response::new(StatusCode::BadRequest)),
    };
}