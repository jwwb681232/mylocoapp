use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "jobs",
            &[
                ("id", ColType::PkAuto),
                ("employer_admin_id",ColType::Integer),
                ("industry_id",ColType::Integer),
                ("title",ColType::String),
                ("description",ColType::Text),
                ("demand",ColType::Unsigned),
                ("start_time",ColType::DateTime),
                ("end_time",ColType::DateTime),
                ("hourly_rate",ColType::Decimal),
                ("required_gender",ColType::SmallUnsigned),
                ("required_ages",ColType::Json),
                ("required_languages",ColType::Json),
                ("required_nationalities",ColType::Json),
                ("status",ColType::SmallUnsigned),
            ],
            &[
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "jobs").await
    }
}
