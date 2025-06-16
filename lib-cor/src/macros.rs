#[macro_export]
// Создание цепочки обработчиков
macro_rules! cor_create {
    ($name:expr, $ctx_type:ty, {$($h:expr),*}) => {{
        let steps = vec![
            $(Box::new($h) as Box<StepFn<$ctx_type>>),*
        ];
        cor::Cor::new(steps)
    }}
}

#[macro_export]
macro_rules! cor_handler {
    ($desc:expr, $ctx_type:ty, $f:expr) => {{
        move |ctx: cor::CtxWrapper<$ctx_type>| -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + Sync + 'static>> {
            Box::pin(async move {
                eprintln!("{}: {:?}", $desc, ctx);
                $f(ctx).await
            })
        }
    }};
}
