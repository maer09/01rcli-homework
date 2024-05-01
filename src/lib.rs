mod cli;
mod utils;

pub use cli::*;
pub use utils::*;
use enum_dispatch::enum_dispatch;

/// The trait for command execution
#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}