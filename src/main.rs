use init::server_init::server_init_proc;
use tracing::info;

// modules tree
pub mod domain {
    pub mod user;
}
pub mod dto {
    pub mod common {}
    pub mod requests {
        pub mod user {
            pub mod signup_request;
        }
    }
    pub mod responses {
        pub mod response_data;
        pub mod response_meta;
    }
}
pub mod errors {

    pub mod code_error;
}
pub mod handlers {
    pub mod user {

        pub mod signup;
    }
    pub mod fallback;
    pub mod root;
}
pub mod routers {
    pub mod middleware {
        pub mod logging;
    }
    pub mod main_router;
}
pub mod init {
    pub mod config;
    pub mod server_init;
    pub mod state;
    pub mod compile_regex;
}
pub mod jobs {}
pub mod util {
    pub mod duration_formatter;
    pub mod now;
}

// main function
#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let start = tokio::time::Instant::now();
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_target(false)
        .init();

    info!("Initializing server...");
    server_init_proc(start).await?;

    Ok(())
}
