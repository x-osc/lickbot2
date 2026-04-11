use std::pin::Pin;

use azalea::Client;
use tokio::task::JoinHandle;

use crate::State;

pub enum TaskStatus {
    Continue,
    Finished,
    Push(Box<dyn Task>),
}

impl TaskStatus {
    pub fn is_finished(&self) -> bool {
        match self {
            Self::Finished => true,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct TaskContext {
    pub bot: Client,
    pub state: State,
}

pub trait Task: Send {
    fn launch(&mut self, ctx: TaskContext);

    fn tick(&mut self, ctx: TaskContext) -> TaskStatus;

    fn pause(&mut self, ctx: TaskContext) {
        self.cancel(ctx);
    }

    fn resume(&mut self, ctx: TaskContext) {
        self.launch(ctx);
    }

    fn cancel(&mut self, ctx: TaskContext);
}

impl Task for Box<dyn Task> {
    fn launch(&mut self, ctx: TaskContext) {
        (**self).launch(ctx)
    }
    fn tick(&mut self, ctx: TaskContext) -> TaskStatus {
        (**self).tick(ctx)
    }
    fn pause(&mut self, ctx: TaskContext) {
        (**self).pause(ctx);
    }
    fn resume(&mut self, ctx: TaskContext) {
        (**self).resume(ctx);
    }
    fn cancel(&mut self, ctx: TaskContext) {
        (**self).cancel(ctx)
    }
}

pub type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

pub struct AsyncTask {
    // needs option because we cant take ownership of factory twice
    // because launch() could technically be run multiple times
    // we need to make running it again do nothing
    //
    // TODO: maybe make into enum
    factory: Option<Box<dyn FnOnce(TaskContext) -> BoxFuture + Send>>,
    handle: Option<JoinHandle<()>>,
}

impl AsyncTask {
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(TaskContext) -> BoxFuture + Send + 'static,
    {
        Self {
            factory: Some(Box::new(f)),
            handle: None,
        }
    }
}

#[macro_export]
macro_rules! async_task {
    (|$ctx:ident| $body:expr) => {
        $crate::task::AsyncTask::new(
            |$ctx: $crate::task::TaskContext| -> $crate::task::BoxFuture {
                Box::pin(async move { $body })
            },
        )
    };
}

impl Task for AsyncTask {
    fn launch(&mut self, ctx: TaskContext) {
        if let Some(factory) = self.factory.take() {
            self.handle = Some(tokio::task::spawn_local(factory(ctx)));
        }
    }

    fn tick(&mut self, _ctx: TaskContext) -> TaskStatus {
        match &self.handle {
            Some(h) if h.is_finished() => TaskStatus::Finished,
            Some(_) => TaskStatus::Continue,
            None => TaskStatus::Continue,
        }
    }

    fn cancel(&mut self, _ctx: TaskContext) {
        if let Some(handle) = self.handle.take() {
            handle.abort();
        }
    }
}
