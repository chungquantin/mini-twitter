macro_rules! impl_global_transaction {
		($($x: ident; feat $feat: expr), *) => {
			use crate::errors::DatabaseError;
			use crate::misc::{ Key, Arg };
			use crate::structures::{FromPostgresRow, KeywordBucket, FromRedisValue};

			#[async_trait::async_trait(?Send)]
			impl crate::structures::SimpleTransaction for Transaction {
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

				async fn set<K, A>(
					&mut self,
					key: K,
					val: A,
					keywords: KeywordBucket
				) -> Result<(), DatabaseError>
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
							} => ds.set(key, val, keywords).await,
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

				async fn get<K, A, V>(
					&self, key: K,
					args: A,
					keywords: KeywordBucket
				) -> Result<Vec<V>, DatabaseError>
				where
								A: Into<Arg> + Send,
								K: Into<Key> + Send,
								V: FromPostgresRow + FromRedisValue
				{
					match self {
						$(
							#[cfg(feature = $feat)]
							Transaction {
								inner: Inner::$x(ds),
								..
							} => ds.get(key, args, keywords).await,
						)*
				}
			}
		}
	}
}
