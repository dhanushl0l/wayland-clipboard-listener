use nix::{
    fcntl::OFlag,
    unistd::{close, dup2_stdin, dup2_stdout, fork, ForkResult},
};
use wayland_clipboard_listener::{WlClipboardCopyStream, WlClipboardListenerError};

use std::io::{stdin, Read};

fn main() -> Result<(), WlClipboardListenerError> {
    let args = std::env::args();
    let context = {
        let len = args.len();
        if len != 2 {
            let mut context = vec![];
            stdin().lock().read_to_end(&mut context).unwrap();
            context
        } else {
            args.last().unwrap().as_bytes().to_vec()
        }
    };
    if context.is_empty() {
        eprintln!("You need to pass something in");
        return Ok(());
    }

    let mimetypes = vec![
        "text/plain",
        "TEXT",
        "UTF8_STRING",
        "text/plain;charset=utf-8",
        "image/png",
    ];
    let mut stream = WlClipboardCopyStream::init()?;

    if let Ok(ForkResult::Child) = unsafe { fork() } {
        if let Ok(dev_null) =
            nix::fcntl::open("/dev/null", OFlag::O_RDWR, nix::sys::stat::Mode::empty())
        {
            let _ = dup2_stdin(&dev_null);
            let _ = dup2_stdout(&dev_null);
            let _ = close(dev_null);
            stream.copy_to_clipboard(context, mimetypes, false)?;
        }
    }

    Ok(())
}
