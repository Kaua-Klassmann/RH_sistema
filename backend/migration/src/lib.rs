pub use sea_orm_migration::prelude::*;

mod m20250718_172400_create_resume_table;
mod m20250718_173430_create_sector_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250718_173430_create_sector_table::Migration),
            Box::new(m20250718_172400_create_resume_table::Migration),
        ]
    }
}
