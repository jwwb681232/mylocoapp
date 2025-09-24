#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20250924_025249_articles;
mod m20250924_053408_comments;
mod m20250924_085032_jobs;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20250924_025249_articles::Migration),
            Box::new(m20250924_053408_comments::Migration),
            Box::new(m20250924_085032_jobs::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}