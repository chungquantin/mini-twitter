macro_rules! impl_repository {
    ($c_name: ident) => {
        use std::cell::Cell;
        use $crate::storage::{Database, DatabaseRef, Transaction};

        pub struct $c_name<'a> {
            pub ds_ref: Cell<DatabaseRef<'a>>,
        }

        impl<'a> $c_name<'a> {
            pub fn new(ds_ref: DatabaseRef<'a>) -> Self {
                $c_name {
                    ds_ref: Cell::new(ds_ref),
                }
            }

            fn db(&mut self) -> &mut Database {
                self.ds_ref.get_mut().db
            }

            pub fn tx(&mut self) -> Transaction {
                self.db().transaction(false).unwrap()
            }

            pub fn mut_tx(&mut self) -> Transaction {
                self.db().transaction(true).unwrap()
            }
        }
    };
}
