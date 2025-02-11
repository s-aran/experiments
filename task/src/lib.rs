mod task;

use std::collections::VecDeque;
use std::sync::Arc;

use uuid::Uuid;

pub struct Task<T, U> {
    uuid: Uuid,
    before: Option<Uuid>,
    after: Option<Uuid>,
    data: T,

    do_callback: Arc<dyn Fn(&mut Self, &U) -> bool>,
    do_dry_run: Box<dyn Fn(&Self, &U) -> bool>,
    called: u32,
}

impl<T, U> Task<T, U> {
    pub fn new(data: T) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            before: None,
            after: None,
            data,
            do_callback: Arc::new(|_, _| true),
            do_dry_run: Box::new(|_, _| true),
            called: 0,
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn call(&mut self, data: U) -> bool {
        self.called += 1;
        let cb = ((self as &mut Self).do_callback).clone();
        cb(self, &data)
    }

    pub fn dry_run(&self, data: U) -> bool {
        ((self as &Self).do_dry_run)(self, &data)
    }

    pub fn called(&self) -> u32 {
        self.called
    }

    pub fn set_callback(&mut self, callback: impl Fn(&mut Self, &U) -> bool + 'static) {
        self.do_callback = Arc::new(callback);
    }

    pub fn set_dry_run_callback(&mut self, callback: impl Fn(&Self, &U) -> bool + 'static) {
        self.do_dry_run = Box::new(callback);
    }

    pub fn update_before(&mut self, uuid: Option<Uuid>) -> Option<Uuid> {
        let result = self.before;

        self.before = uuid;

        result
    }

    pub fn update_after(&mut self, uuid: Option<Uuid>) -> Option<Uuid> {
        let result = self.after;

        self.after = uuid;

        result
    }
}

pub struct TaskManager<T, U> {
    tasks: VecDeque<Task<T, U>>,
}

impl<T, U> TaskManager<T, U> {
    pub fn new() -> Self {
        Self {
            tasks: VecDeque::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn clear(&mut self) {
        self.tasks.clear();
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn push(&mut self, mut task: Task<T, U>) {
        if !self.is_empty() {
            let last = self.tasks.back_mut().unwrap();

            last.update_after(Some(task.uuid()));
            task.update_before(Some(last.uuid()));
        }

        self.tasks.push_back(task);
    }

    pub fn pop(&mut self) -> Option<Task<T, U>> {
        let result = self.tasks.pop_front();

        if !self.is_empty() {
            let first = self.tasks.front_mut().unwrap();

            first.update_before(None);
        }

        result
    }

    pub fn pop_and_call(&mut self, data: U) -> (Option<Task<T, U>>, bool) {
        let mut result = self.pop();

        let called = if let Some(task) = result.as_mut() {
            task.call(data)
        } else {
            false
        };

        (result, called)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid() {
        let task1 = Task::<_, i32>::new(1);
        let task2 = Task::<_, i32>::new(1);

        assert_ne!(task1.uuid, task2.uuid);
    }

    #[test]
    fn test_len() {
        let mut manager = TaskManager::new();

        assert_eq!(0, manager.len());

        manager.push(Task::new(1));
        manager.push(Task::new(2));
        manager.push(Task::new(3));
        manager.push(Task::new(4));

        assert_eq!(4, manager.len());

        manager.pop();

        assert_eq!(3, manager.len());

        manager.clear();

        assert_eq!(0, manager.len());

        manager.pop_and_call(1);
        assert_eq!(0, manager.len());
    }

    #[test]
    fn test_call() {
        let mut task = Task::new(1);

        assert_eq!(0, task.called());
        assert_eq!(&1, task.data());

        assert_eq!(true, task.call(1));

        assert_eq!(1, task.called());

        task.set_callback(|this, arg| {
            assert_eq!(2, *arg);

            this.data = 3;

            false
        });

        assert_eq!(false, task.call(2));
        assert_eq!(&3, task.data());
        assert_eq!(1, Arc::strong_count(&task.do_callback));
    }

    #[test]
    fn test_dry_run() {
        let mut task = Task::new(1);

        assert_eq!(0, task.called());

        assert_eq!(true, task.dry_run(1));

        assert_eq!(0, task.called());

        task.set_dry_run_callback(|this, arg| {
            assert_eq!(2, *arg);

            false
        });

        assert_eq!(false, task.dry_run(2));
        assert_eq!(1, Arc::strong_count(&task.do_callback));
    }
}
