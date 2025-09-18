use std::sync::{Arc, Mutex};

struct SafeCounter {
    value: Mutex<i32>,
}

impl SafeCounter {
    fn increment(&self) {
        let mut lock = self.value.lock().unwrap();
        *lock += 1;
    } // ✅ Function finishes, but...
}

#[tokio::main]
async fn main() {
    let counter = Arc::new(SafeCounter::new());
    
    // 🔥 THIS is why we need mutex!
    // Multiple tasks sharing the SAME counter
    let mut handles = vec![];
    
    for i in 0..1000 {
        let counter_clone = counter.clone(); // Shared ownership
        let handle = tokio::spawn(async move {
            // Multiple tasks calling increment() SIMULTANEOUSLY
            counter_clone.increment(); // ← Race condition without mutex!
            
            some_async_work().await; // Task yields here
            
            counter_clone.increment(); // ← Another potential race!
        });
        handles.push(handle);
    }
    
    // All 1000 tasks are running concurrently!
    for handle in handles {
        handle.await.unwrap();
    }
}
