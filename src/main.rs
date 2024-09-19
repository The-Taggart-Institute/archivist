use poise::serenity_prelude as serenity;

mod wallabag;
use wallabag::add_article;
mod config;
use config::load_config;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Status
#[poise::command(
    slash_command,
    required_permissions = "ADMINISTRATOR"
)]
async fn status(ctx: Context<'_>) -> Result<(), Error> {
    let response = format!("Everything's a-ok 👍");
    ctx.say(response).await?;
    Ok(())
}

/// Add Article
#[poise::command(
    slash_command,
    required_permissions = "MANAGE_MESSAGES"
)]
async fn archive(ctx: Context<'_>, 
    #[description = "URL of article"] url: String,
    #[description = "comma-separated list of tags"] tags: Option<String>

) -> Result<(), Error> {
    let config = load_config().unwrap();
    let tags = tags.unwrap_or(String::new());
    match add_article(&url, &tags, &config).await {
        Ok(_) => {
            let mut reply = format!("Added {url}");
            if !tags.is_empty() {
                reply.push_str(" with tags ");
                for t in tags.split(',').collect::<Vec<&str>>() {
                    reply.push_str(format!("`{t}` ").as_str());
                }
            }
            ctx.say(reply).await?;
            Ok(())
        },
        Err(_) => {
            ctx.say("Uh oh").await?;
            Ok(())

        }
    }
}

/// Register new commands button
#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let config = load_config().unwrap();
    let token = config.discord_token;
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![status(), register(), archive()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
