use db::database::ArrayField;
use util::{data_printer, collection_printer, database_printer};

use crate::db::database::{Database, Collection, Data};

mod db;
mod util;

fn main() {

    let db_name = "db teste".to_string();
    let mut db: Database = Database::new(db_name);

    let collection1 = Collection::new("collection 1".to_string());
    let collection2 = Collection::new("collection 2".to_string());
    let collection3 = Collection::new("collection 3".to_string());
    let mut collection4 = Collection::new("collection 4".to_string());

    let mut array_data: Vec<String> = Vec::new(); 
    array_data.push("arroz".to_string());
    array_data.push("feijão".to_string());
    array_data.push("batata frita".to_string());

    let array_field: ArrayField = ArrayField::new("ingredientes: ".to_string(), array_data);

    let data: Data = Data::new(array_field);

    collection4.content.push(data.clone());

    db.collections.push(collection1);
    db.collections.push(collection2);
    db.collections.push(collection3);
    db.collections.push(collection4.clone());

    database_printer(&db);
}
