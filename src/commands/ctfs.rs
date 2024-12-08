use crate::{
    types::global::{Data, Error},
    utils::ctf_sender::send_ctf,
};

#[poise::command(slash_command)]
pub async fn send(
    ctx: poise::ApplicationContext<'_, Data, Error>,
    #[description = "CTFTime URL"] url: String,
) -> Result<(), Error> {
    send_ctf(ctx, url).await
}
