use std::{error::Error, sync::Arc};
use tokio::sync::RwLock;
use zmc::network::{context::ServerContext, protocol::MinecraftProtocol};
use zr_protocol::{
    event::ServerEvents,
    server::{Config, Server},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let protocol = Arc::new(RwLock::new(MinecraftProtocol::<ServerContext>::default()));
    let on_join_protocol = protocol.clone();
    let on_message_protocol = protocol.clone();
    let on_quit_protocol = protocol.clone();
    let server = Server::new("127.0.0.1:25565", protocol.clone())
        .await?
        .config(Config::new())
        .events(
            ServerEvents::new()
                .on_join(
                    async move |ctx: zr_protocol::context::Context<
                        MinecraftProtocol<ServerContext>,
                    >| {
                        println!(
                            "[\x1b[32m+\x1b[m] {}",
                            ctx.peer_addr()
                                .map(|addr| addr.to_string())
                                .unwrap_or_default()
                        );
                        on_join_protocol.write().await.add_client(ctx);
                    },
                )
                .on_quit(
                    async move |ctx: zr_protocol::context::Context<
                        MinecraftProtocol<ServerContext>,
                    >| {
                        println!(
                            "[\x1b[31m-\x1b[m] {}",
                            ctx.peer_addr()
                                .map(|addr| addr.to_string())
                                .unwrap_or_default()
                        );
                        on_quit_protocol.write().await.add_client(ctx);
                    },
                )
                .on_message(async move |ctx| {
                    on_message_protocol.write().await.on_message(ctx).await
                }),
        );
    server.run().await?;
    Ok(())
}
