pub mod build;
pub mod info;
pub mod sig;
pub mod run;

pub trait SurrealismCommand {
    fn run(self) -> anyhow::Result<()>;
}