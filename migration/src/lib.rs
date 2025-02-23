pub use sea_orm_migration::prelude::*;

mod m20241014_134117_create_table_mapping_url;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(
            m20241014_134117_create_table_mapping_url::Migration,
        )]
    }
}
