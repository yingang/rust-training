pub mod user;
pub mod strtok;
pub mod ticket;
pub mod fibonacci;
pub mod encoder;
pub mod actor;  // 如果这里不用pub，会产生更多定义未使用方面的warning

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[test]
fn drop_test() {
    struct MyString(String);
    struct MyBox<T>(Box<T>);
    struct MyVec<T>(Vec<T>);
    impl Drop for MyString {
        fn drop(&mut self) {
            println!("MyString {} dropped", self.0);
        }
    }
    impl<T> Drop for MyBox<T> {
        fn drop(&mut self) {
            println!("MyBox dropped");
        }
    }
    impl<T> Drop for MyVec<T> {
        fn drop(&mut self) {
            println!("MyVec dropped");
        }
    }

    let a = MyString("A".to_string());
    let b = MyString("B".to_string());
    let v = MyVec(vec![MyBox(Box::new(a)), MyBox(Box::new(b))]);
    drop(v);
}

#[test]
fn fat_pointer_test() {
    use std::mem::transmute;
    use std::mem::size_of;
    
    println!("{:?}", size_of::<&[i32; 3]>());
    println!("{:?}", size_of::<&[i32]>());

    let v : [i32; 5] = [1,2,3,4,5];
    let p : &[i32] = &v[2..4];
    unsafe {
        let (ptr, len) : (usize, isize) = transmute(p);
        println!("{} {}", ptr, len);

        let ptr = ptr as *const i32;
        for i in 0..len {
            println!("{}", *ptr.offset(i));
        }
    }
}