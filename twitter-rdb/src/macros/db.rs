macro_rules! impl_new_database {
    ($DbType: ty) => {
        #[allow(dead_code)]
        pub fn get_inner(self: &Self) -> &DatabaseAdapter<$DbType> {
            &self.0
        }

        #[allow(dead_code)]
        pub fn get_initialized_inner(
            self: &Self,
        ) -> Result<&DatabaseAdapter<$DbType>, DatabaseError> {
            let core = self.get_inner();
            if Some(&core.db_instance).is_none() {
                return Err(DatabaseError::DbNotInitialized);
            }
            Ok(core)
        }

        #[allow(dead_code)]
        pub fn get_mut_inner(self: &mut Self) -> &mut DatabaseAdapter<$DbType> {
            &mut self.0
        }

        #[allow(dead_code)]
        pub fn get_mut_initialized_inner(
            self: &mut Self,
        ) -> Result<&mut DatabaseAdapter<$DbType>, DatabaseError> {
            let core = self.get_mut_inner();
            if Some(&core.db_instance).is_none() {
                return Err(DatabaseError::DbNotInitialized);
            }
            Ok(core)
        }
    };
}
