use std::collections::VecDeque;

use uuid::Uuid;

pub trait Task<T, U, R, E> {
    fn uuid(&self) -> Uuid;
    fn execute(&mut self, data: &mut T, arg: &mut U) -> Result<R, E>;
    fn rollback(&mut self, data: &mut T, arg: &mut U) -> Result<R, E>;
}

pub struct Invoker<'a, T: 'a, U: 'a, R: 'a, E: 'a> {
    tasks: VecDeque<Box<dyn Task<T, U, R, E> + 'a>>,
    data: &'a mut T,
    current_uuid: Option<Uuid>,
}

impl<'a, T, U, R, E> Invoker<'a, T, U, R, E>
where
    R: Default,
{
    pub fn new(data: &'a mut T) -> Self {
        Self {
            tasks: VecDeque::new(),
            data,
            current_uuid: None,
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn set_data(&mut self, data: &'a mut T) -> &mut Self {
        self.data = data;

        self
    }

    pub fn clear(&mut self) -> &mut Self {
        self.tasks.clear();
        self.current_uuid = None;

        self
    }

    pub fn push<X: Task<T, U, R, E> + 'a>(&mut self, task: X) -> &mut Self {
        self.tasks.push_back(Box::new(task));

        self
    }

    pub fn pop(&mut self) -> Option<Box<dyn Task<T, U, R, E> + 'a>> {
        self.tasks.pop_front()
    }

    pub fn index(&self) -> usize {
        match self.current_uuid {
            Some(uuid) => {
                for (i, ele) in self.tasks.iter().enumerate() {
                    if ele.uuid() == uuid {
                        return i;
                    }
                }
                return 0;
            }
            None => 0,
        }
    }

    pub fn execute(&mut self, arg: &mut U) -> Result<R, E> {
        if self.tasks.len() <= 0 {
            // NOP
            return Ok(R::default());
        }

        let current_index = self.index();
        let c = self.tasks.get_mut(current_index).unwrap();
        // let t = &mut *self.data;

        let result = c.execute(&mut self.data, arg);

        if result.is_err() {
            return result;
        }

        self.current_uuid = Some(c.uuid());

        result
    }

    pub fn rollback(&mut self, arg: &mut U) -> Result<R, E> {
        if self.tasks.len() <= 0 {
            // NOP
            return Ok(R::default());
        }

        let current_index = self.index();
        let c = self.tasks.get_mut(current_index).unwrap();
        // let t = &mut *self.data;

        let result = c.rollback(&mut self.data, arg);

        if result.is_err() {
            return result;
        }

        self.current_uuid = Some(c.uuid());

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    struct Target {
        val: i32,
    }

    impl Target {
        fn new() -> Self {
            Self { val: 0 }
        }

        fn get(&self) -> i32 {
            self.val
        }

        fn set(&mut self, new_val: i32) -> &mut Self {
            self.val = new_val;

            self
        }
    }

    struct UpdateOneTask;

    impl Task<Target, i32, bool, ()> for UpdateOneTask {
        fn uuid(&self) -> Uuid {
            Uuid::new_v4()
        }

        fn execute(&mut self, data: &mut Target, arg: &mut i32) -> Result<bool, ()> {
            data.val = *arg;
            Ok(true)
        }

        fn rollback(&mut self, data: &mut Target, arg: &mut i32) -> Result<bool, ()> {
            data.val = *arg;

            Ok(false)
        }
    }

    struct UpdateTwoTask;

    impl Task<Target, i32, bool, ()> for UpdateTwoTask {
        fn uuid(&self) -> Uuid {
            Uuid::new_v4()
        }

        fn execute(&mut self, data: &mut Target, arg: &mut i32) -> Result<bool, ()> {
            data.val = *arg;
            Ok(true)
        }

        fn rollback(&mut self, data: &mut Target, arg: &mut i32) -> Result<bool, ()> {
            data.val = *arg;
            Ok(true)
        }
    }

    #[test]
    fn test() {
        let mut target = Target::new();
        let mut invoker = Invoker::new(&mut target);

        assert_eq!();

        invoker.push(UpdateOneTask);
        let mut truthy = 1;
        assert!(invoker.execute(truthy).is_ok());
        assert_eq!(true, targ);

        invoker.push(UpdateTwoTask);
        assert!(invoker.execute(&0));
        assert_eq!(false, invoker.data().get());
    }
}
