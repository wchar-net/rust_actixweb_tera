mod init;
use dotenv::dotenv;
use init::create_server;
use std:: {env,net::SocketAddr,str::FromStr};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let server_ip = match env::var("SERVER_IP") {
        Ok(ip) => match ip.is_empty() {
            true => "NO_IP_CONFIGURED".to_string(),
            false => match ip.len() < 8 {
                true => "INVALID_IP_LENGTH".to_string(),
                false => match SocketAddr::from_str(&ip).is_ok() {
                    true => ip,
                    false => "INVALID_IP".to_string(),
                },
            },
        },
        Err(_) => "NO_IP_CONFIGURED".to_string(),
    };
    let _server = create_server(&server_ip).await?;
    Ok(())
}
