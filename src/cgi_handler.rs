use futures::Stream;
use tokio_io::{io, AsyncRead, AsyncWrite};
use tokio_io::codec::FramedRead;
use tokio_file_unix::{File, StdFile};

fn start() -> std::io::Result<()> {
    // initialize the event loop
    let mut core = tokio_core::reactor::Core::new()?;
    let handle = core.handle();

    // get the standard io as a file
    let stdin = stdio::stdin();
    let stdout = stdio::stdout();
    let stderr = stdio::stderr();
    let reader = File::new_nb(StdFile(stdin.lock()))?.into_reader(&handle)?;
    let mut writer = File::new_nb(StdFile(stdout.lock()))?.into_io(&handle)?;
    let mut err = File::new_nb(StdFile(stderr.lock()))?.into_io(&handle)?;

    // turn it into a stream of lines and process them
    let future = io::lines(reader).for_each(|line| {
        match handle_message(line) {
                Ok(output) => {
                    match output {
                        Some(o) => writeln!(writer, "{}", o),
                        None => Ok(()),
                    }
                }
                Err(e) => writeln!(err, 
                     "Status: 500\r\n\r\n
                     <h1>500 Internal Server \
                      Error</h1>
                     <p>{}</p>",
                     e),
            }
            .map(|_| ())
    });

    // start the event loop
    core.run(future)?;
    Ok(())
}
