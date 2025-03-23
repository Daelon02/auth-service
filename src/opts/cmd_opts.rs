use serde::Deserialize;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Deserialize)]
pub struct Opts {
    #[structopt(flatten)]
    pub application: ApplicationOpts,
    #[structopt(flatten)]
    pub database: DatabaseOpts,
    #[structopt(flatten)]
    pub auth0: Auth0Opts,
}

#[derive(StructOpt, Debug, Deserialize)]
pub struct ApplicationOpts {
    #[structopt(long, env = "BIND", default_value = "localhost:8080")]
    pub bind: String,
}

#[derive(StructOpt, Debug, Deserialize)]
pub struct DatabaseOpts {
    #[structopt(long, env = "DATABASE_URL")]
    pub database_url: String,
}

#[derive(StructOpt, Debug, Deserialize)]
pub struct Auth0Opts {
    #[structopt(long, env = "CLIENT_ID", default_value = "admin")]
    pub client_id: String,
    #[structopt(long, env = "CLIENT", default_value = "localhost:8080")]
    pub client: String,
    #[structopt(long, env = "CLIENT_SECRET", default_value = "admin")]
    pub client_secret: String,
    #[structopt(
        long,
        env = "CONNECTION",
        default_value = "Username-Password-Authentication"
    )]
    pub connection: String,
}
