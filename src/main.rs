use futures::prelude::*;
use irc::client::prelude::*;

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    // We can also load the Config at runtime via Config::load("path/to/config.toml")
    let config = Config {
        nickname: Some("nagbot".to_owned()),
        server: Some("irc.z.je".to_owned()),
        channels: vec!["#ant.org".to_owned()],
        ..Config::default()
    };

    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;

    while let Some(message) = stream.next().await.transpose()? {
        print!("{}", message);
        if let Command::PRIVMSG(channel, _msg) = message.command {
            client
                .send_privmsg(
                    &channel,
                    "Shut up! We have moved to irc://irc.libera.chat:6697/#ant.org - go there now!",
                )
                .unwrap();
        }
    }

    Ok(())
}
