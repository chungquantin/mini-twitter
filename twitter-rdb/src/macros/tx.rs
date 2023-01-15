macro_rules! impl_global_transaction {
		($($x: ident; feat $feat: expr), *) => {
			use crate::errors::DatabaseError;
			use crate::misc::{ Key, Val, Arg };

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

				async fn set<K, V>(&mut self, key: K, val: V) -> Result<(), DatabaseError>
				where
					K: Into<Key> + Send,
					V: Into<Val> + Send
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

				async fn get_filtered<K, A>(&self, key: K, args: A) -> Result<Val, DatabaseError>
				where
								A: Into<Arg> + Send,
								K: Into<Key> + Send,
				{
					match self {
						$(
							#[cfg(feature = $feat)]
							Transaction {
								inner: Inner::$x(ds),
								..
							} => ds.get_filtered(key, args).await,
						)*
				}
			}
		}
	}
}
