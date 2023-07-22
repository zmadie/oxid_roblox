use async_trait::async_trait;

#[async_trait(?Send)]
pub(crate) trait ResultExtensions<T, E> {
    async fn map_async<U, Future, Mapper>(self, mapper: Mapper) -> Result<U, E>
    where
        Future: std::future::Future<Output = U> + Send,
        Mapper: FnOnce(T) -> Future;
}

#[async_trait(?Send)]
impl<T, E> ResultExtensions<T, E> for Result<T, E> {
    async fn map_async<U, Future, Mapper>(self, mapper: Mapper) -> Result<U, E>
    where
        Future: std::future::Future<Output = U> + Send,
        Mapper: FnOnce(T) -> Future,
    {
        match self {
            Ok(value) => Ok(mapper(value).await),
            Err(value) => Err(value),
        }
    }
}
