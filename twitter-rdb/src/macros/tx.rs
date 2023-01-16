macro_rules! impl_global_transaction {
		($($x: ident; feat $feat: expr), *) => {
			use crate::errors::DatabaseError;
			use crate::misc::{ Key, Arg };
			use crate::models::FromSuperValues;

			#[async_trait::async_trait(?Send)]
			impl crate::models::SimpleTransaction for Transaction {
				// Check if closed
				fn closed(&self) -> bool {
					match self {
						$(
							#[cfg(feature = $feat)]
							Transaction {
								inner: Inner::$x(ds),
								..
							} => ds.closed(),
						)*
					}
				}

				// Cancel a transaction
				async fn cancel(&mut self) -> Result<(), DatabaseError> {
					match self {
						$(
							#[cfg(feature = $feat)]
							Transaction {
								inner: Inner::$x(ds),
								..
							} => ds.cancel().await,
						)*
					}
				}

				// Commit a transaction
				async fn commit(&mut self) -> Result<(), DatabaseError> {
					match self {
						$(
							#[cfg(feature = $feat)]
							Transaction {
								inner: Inner::$x(ds),
								..
							} => ds.commit().await,
						)*
					}
				}

				async fn set<K, A>(&mut self, key: K, val: A) -> Result<(), DatabaseError>
				where
					K: Into<Key> + Send,
					A: Into<Arg> + Send
				{
					match self {
						$(
							#[cfg(feature = $feat)]
							Transaction {
								inner: Inner::$x(ds),
								..
							} => ds.set(key, val).await,
						)*
					}
				}

				async fn multi_set<K, A>(&mut self, keys: K, args: Vec<A>) -> Result<(), DatabaseError>
				where
					K: Into<Key> + Send,
					A: Into<Arg> + Send
				{
					match self {
						$(
							#[cfg(feature = $feat)]
							Transaction {
								inner: Inner::$x(ds),
								..
							} => ds.multi_set(keys, args).await,
						)*
					}
				}

				async fn get_filtered<K, A, V>(
					&self, key: K,
					args: A,
					keywords: &[&'static str]
				) -> Result<Vec<V>, DatabaseError>
				where
								A: Into<Arg> + Send,
								K: Into<Key> + Send,
								V: FromSuperValues
				{
					match self {
						$(
							#[cfg(feature = $feat)]
							Transaction {
								inner: Inner::$x(ds),
								..
							} => ds.get_filtered(key, args, keywords).await,
						)*
				}
			}
		}
	}
}
