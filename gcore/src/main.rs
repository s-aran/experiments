struct HitBox {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl HitBox {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn get_rb(&self) -> (i32, i32) {
        (self.x + self.width as i32, self.y + self.height as i32)
    }

    pub fn is_hit(a: &HitBox, b: &HitBox) -> bool {
        let a_left = a.x;
        let a_top = a.y;
        let a_right = a.width as i32 + a.x;
        let a_bottom = a.height as i32 + a.y;

        let b_left = b.x;
        let b_top = b.y;
        let b_right = b.width as i32 + b.x;
        let b_bottom = b.height as i32 + b.y;

        (a_left <= b_right && b_left <= a_right) && (a_top <= b_bottom && b_top <= a_bottom)
    }
}

trait Move {
    fn rel_to(&mut self, x: i32, y: i32) -> Self;
    fn abs_to(&mut self, x: i32, y: i32) -> Self;
}

trait Render {
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn zoom(&self) -> u16;
    fn render(&self, x: i32, y: i32) -> Self;
}

trait Gravity {
    fn gravity(&self) -> i32;
}

trait Acceleration {
    fn acceleration(&self) -> (i32, i32);
}

trait Counter<T> {
    fn get(&self) -> T;
    fn count(&mut self) -> T;

    fn set(&mut self) -> Self;
    fn init_value(&self) -> T;
    fn reset(&self) -> Self;
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hitbox() {
        {
            let a = HitBox::new(0, 0, 100, 100);
            let b = HitBox::new(100, 100, 100, 100);

            assert!(HitBox::is_hit(&a, &b));
            assert!(HitBox::is_hit(&b, &a));
        }

        {
            let a = HitBox::new(0, 100, 100, 100);
            let b = HitBox::new(100, 100, 100, 100);

            assert!(HitBox::is_hit(&a, &b));
            assert!(HitBox::is_hit(&b, &a));
        }

        {
            let a = HitBox::new(100, 0, 100, 100);
            let b = HitBox::new(100, 100, 100, 100);

            assert!(HitBox::is_hit(&a, &b));
            assert!(HitBox::is_hit(&b, &a));
        }

        {
            let a = HitBox::new(100, 100, 100, 100);
            let b = HitBox::new(100, 100, 100, 100);

            assert!(HitBox::is_hit(&a, &b));
            assert!(HitBox::is_hit(&b, &a));
        }
    }

    #[test]
    fn test_hitbox_no_collision() {
        {
            let a = HitBox::new(-1, -1, 100, 100);
            let b = HitBox::new(100, 100, 100, 100);

            assert!(!HitBox::is_hit(&a, &b));
            assert!(!HitBox::is_hit(&b, &a));
        }

        {
            let a = HitBox::new(-1, 100, 100, 100);
            let b = HitBox::new(100, 100, 100, 100);

            assert!(!HitBox::is_hit(&a, &b));
            assert!(!HitBox::is_hit(&b, &a));
        }

        {
            let a = HitBox::new(100, -1, 100, 100);
            let b = HitBox::new(100, 100, 100, 100);

            assert!(!HitBox::is_hit(&a, &b));
            assert!(!HitBox::is_hit(&b, &a));
        }
    }
}
