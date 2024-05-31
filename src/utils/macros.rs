pub struct Defer<F: FnOnce()> {
    deferred: Option<F>,
}

impl<F: FnOnce()> Defer<F> {
    pub fn new(deferred: F) -> Self {
        Self {
            deferred: Some(deferred),
        }
    }
}

impl<F: FnOnce()> Drop for Defer<F> {
    fn drop(&mut self) {
        if let Some(deferred) = self.deferred.take() {
            deferred();
        }
    }
}

#[macro_export]
macro_rules! defer {
    ($e:expr) => {
        let _defer = $crate::utils::macros::Defer::new(|| $e);
    };
}

// fn main() {
//     let x = 42u8;
//     defer!(println!("defer 1"));
//     defer!({
//         println!("defer 2");
//         println!("inside defer {}", x)
//     });
//     println!("normal execution {}", x);
// }
