use crate::db::database::{ArrayField, Collection, Data, Database};

pub fn database_printer(db: &Database) {

    println!("{} {{", db.name);
    for i in 0..db.collections.len() {
        if i == db.collections.len() - 1 {
            collection_printer(&db.collections[i]);
        } else {
            collection_printer(&db.collections[i]);
        }
    }

    println!("}}");
}

pub fn collection_printer(cl: &Collection) {
    println!("  {}    {{", cl.name);
    if cl.content.is_empty() {
        println!("  }},");
    }

    for i in 0..cl.content.len() {
        if i == cl.content.len() - 1 {
            data_printer(&cl.content[i]);
            println!("  }}");
        } else {
            data_printer(&cl.content[i]);
            println!("  }},");
        }
    }

}

pub fn data_printer(dt: &Data) {
    println!("      id: {},", dt.id);

    array_data_printer(&dt.data);
}

fn array_data_printer(array: &ArrayField) {
    println!("      {}: [", array.name);
    for i in 0..array.array.len() {
        if i == array.array.len() - 1 {
            println!("          {}", array.array[i]);
        } else {
            println!("          {},", array.array[i]);
        }
    }
    println!("      ]");
}
