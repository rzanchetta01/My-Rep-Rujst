pub mod database {

    use uuid::Uuid;

    #[derive(Debug, Clone)]
    pub struct Database {
        pub name: String,
        pub collections: Vec<Collection>,
    }

    impl Database {
        pub fn new(db_name: String) -> Database {
            let collections: Vec<Collection> = Vec::new();

            return Database {
                name: db_name,
                collections,
            };
        }
    }

    #[derive(Debug, Clone)]
    pub struct Collection {
        pub name: String,
        pub content: Vec<Data>,
    }

    impl Collection {
        pub fn new(cl_name: String) -> Collection {
            let content: Vec<Data> = Vec::new();

            return Collection {
                name: cl_name,
                content,
            };
        }
    }

    #[derive(Debug, Clone)]
    pub struct Data {
        pub id: String,
        pub data: ArrayField,
    }

    impl Data {
        pub fn new(data: ArrayField) -> Data {
            let id: String = Uuid::new_v4().to_string();

            return Data { id, data };
        }
    }

    #[derive(Debug, Clone)]
    pub struct ArrayField {
        pub name: String,
        pub array: Vec<String>,
    }

    impl ArrayField {
        pub fn new(name: String, array: Vec<String>) -> ArrayField {
            return ArrayField { name, array };
        }
    }
}
