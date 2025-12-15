pub use sea_orm_migration::prelude::*;

mod m20251214_174342_create_user;
mod m20251214_174342_create_group;
mod m20251214_174342_create_message;
mod m20251214_174342_create_file_msg;
mod m20251214_174342_create_user_group;
mod m20251214_174346_create_conversation;
mod m20251214_174623_create_uuid_extension;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251214_174623_create_uuid_extension::Migration),
            Box::new(m20251214_174342_create_user::Migration),
            Box::new(m20251214_174342_create_group::Migration),
            Box::new(m20251214_174342_create_message::Migration),
            Box::new(m20251214_174342_create_file_msg::Migration),
            Box::new(m20251214_174342_create_user_group::Migration),
            Box::new(m20251214_174346_create_conversation::Migration),
        ]
    }
}
