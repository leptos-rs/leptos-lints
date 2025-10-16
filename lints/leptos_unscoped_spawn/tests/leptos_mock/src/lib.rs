pub mod task {
    pub fn spawn_local<F>(_f: F)
    where
        F: std::future::Future<Output = ()> + 'static,
    {
        // Implementation of spawn_local
    }

    pub fn spawn_local_scoped<F>(_f: F)
    where
        F: std::future::Future<Output = ()> + 'static,
    {
        // Implementation of spawn_local_scoped
    }
}
