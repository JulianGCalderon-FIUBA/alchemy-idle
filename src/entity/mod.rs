pub mod gatherer;
pub mod logger;
pub mod alchemist;
pub mod trader;

pub trait Entity: Sized + Send + 'static {
    fn work(&mut self);
    fn rest(&mut self);
    fn alive(&self) -> bool;

    fn start(mut self) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || {
            while self.alive() {
                self.work();
                self.rest();
            }
        })
    }
}
