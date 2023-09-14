pub mod gatherer;
pub mod logger;

pub trait Entity: Sized + Send + 'static {
    fn work(&mut self);
    fn rest(&mut self);

    fn start(mut self) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || loop {
            self.work();
            self.rest();
        })
    }
}
