#[salsa::db]
#[derive(Default)]
pub struct PuppyDatabaseImpl {
    storage: salsa::Storage<Self>,
}

#[salsa::db]
impl salsa::Database for PuppyDatabaseImpl {}
