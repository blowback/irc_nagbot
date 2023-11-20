use futures::prelude::*;
use irc::client::prelude::*;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::time::{Duration, Instant};

const TIMEOUT: Duration = Duration::new(5u64 * 60u64, 0u32); // PROD 5 mins cool down
                                                             //const TIMEOUT: Duration = Duration::new(10u64, 0u32); // TEST 10 secs cool down

const UTTERANCES: &'static [&'static str] = &[
    "Shut up! We have moved to irc://irc.libera.chat:6697/#ant.org - go there now!",
    "Voluble your opinion may be, valuable it is not. Try: irc://irc.libera.chat:6697/#ant.org",
    "Nobody is listening - they are all on irc://irc.libera.chat:6697/#ant.org",
    "Be as gregarious as you are garrulous: move to irc://irc.libera.chat:6697/#ant.org",
    "Perhaps consider moving to irc://irc.libera.chat:6697/#ant.org ?",
    "THE END IS NIGH! Move to irc://irc.libera.chat:6697/#ant.org !",
    "Are you thick or something? We have moved to irc://irc.libera.chat:6697/#ant.org",
    "Whatever, nobody cares. Try irc://irc.libera.chat:6697/#ant.org",
    "There's nobody here but us bots: all the monkeys went to irc://irc.libera.chat:6697/#ant.org",
    "irc://irc.libera.chat:6697/#ant.org : you know it makes sense!",
    "Talking to yourself again? Why not talk in irc://irc.libera.chat:6697/#ant.org",
    "Howling into the void? Howl into irc://irc.libera.chat:6697/#ant.org !",
    "For the love of god, switch to irc://irc.libera.chat:6697/#ant.org",
    "Alone in the dark? Demons ready to torture you at irc://irc.libera.chat:6697/#ant.org",
    "All the lonely people...became less lonely at irc://irc.libera.chat:6697/#ant.org",
    "Since you've been gone...we all went to irc://irc.libera.chat:6697/#ant.org",
    "Ain't no way you'll be lonely...at irc://irc.libera.chat:6697/#ant.org",
    "Dont wanny be all by yourself any more? go to irc://irc.libera.chat:6697/#ant.org",
];

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

    let mut activity: HashMap<String, Instant> = HashMap::new();

    let mut rng = rand::thread_rng();

    while let Some(message) = stream.next().await.transpose()? {
        print!("{}", message);
        if let Command::PRIVMSG(ref channel, ref _msg) = message.command {
            if let Some(sender) = message.response_target() {
                let now = Instant::now();

                // if there was no entry in the hash, or
                // there was an entry in the cache but it is too old,
                // send the nag
                let nag: bool = match activity.insert(sender.to_string(), now) {
                    None => true,
                    Some(prev) if now.duration_since(prev) > TIMEOUT => true,
                    _ => false,
                };

                if nag {
                    client
                        .send_privmsg(&channel, UTTERANCES.choose(&mut rng).unwrap())
                        .unwrap();
                }
            }
        }
    }

    Ok(())
}

// let now = Instant::now();
// let elapsed = now.elapsed();
