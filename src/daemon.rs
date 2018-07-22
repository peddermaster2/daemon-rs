use std::io;
use std::sync::mpsc::Receiver;

/// The state of the daemon
pub enum State {
    /// The daemon is started
    Start,
    /// ?The daemon is reloading
    Reload,
    /// The daemon is stopped
    Stop,
}

/// This struct is the Daemon instance
///
/// Create a instance of this daemon and call the `run` method,
/// which is provided by the `DaemonRunner` trait.
///
/// See the `bin/examples.rs` for usage.
pub struct Daemon {
    /// The name for the daemon
    pub name: String,
}

/// Helps to know how the executable was started
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum IsDaemon {
    /// Started as a service on windows
    Yes,
    /// Started as a normal executable on windows
    No,
    /// Started on linux
    Unknown,
}

/// The trait with the run function
///
/// See the `bin/examples.rs` for usage.
pub trait DaemonRunner {
    /// This function has to be called by the user of the library
    ///
    /// The parameter `IsDaemon` will be `Unknown` on linux.
    /// On windows it will either be `Yes` if started as a service or `No` if started directly.
    fn run<F: 'static + FnOnce(Receiver<State>, IsDaemon)>(&self, f: F) -> io::Result<()>;
}
