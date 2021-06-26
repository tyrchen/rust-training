pub mod actor;
pub mod actor_improved;
pub mod encoder;
pub mod fibonacci;
pub mod strtok;
pub mod ticket;
pub mod user;

#[test]
fn copy_move_test() {
    let a = 10;
    let b = vec![1, 2, 3];
    {
        let (x, y) = (a, b);
    }

    let c = a;
    // let d = b;
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

    let s1 = MyString("Hello world".to_string());
    let s2 = MyString("Goodbye world".to_string());
    let arr = MyVec(vec![MyBox(Box::new(s1)), MyBox(Box::new(s2))]);
    drop(arr);
}
