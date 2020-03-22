pub fn spawn<F, T>(f: F) -> JoinHandle<T> 
where 
    F: FnOnce() -> T,  // 'static make sure that clousure should not reference value from enclosed function
    F: Send + 'static, // Types that can be transferred across thread boundaries.
    T: Send + 'static  
{

}
