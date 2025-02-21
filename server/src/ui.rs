use color_print::ceprintln;
use crossterm::event::{read, Event};
use crossterm::{cursor, terminal, terminal::ClearType, ExecutableCommand};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

//////////////////////////////////////////////////////////////////////
//////////////////////////// public //////////////////////////////////
//////////////////////////////////////////////////////////////////////

pub fn intro() {
    print!("{}", TITLE_BANNER);
    for _ in 0..40 {
        print!(".");
        _ = io::stdout().flush();
        thread::sleep(Duration::from_millis(50));
    }
    _ = clear_screen();
    _ = io::stdout().execute(cursor::MoveTo(0, 1));
}

pub fn clear_screen() -> io::Result<()> {
    _ = io::stdout().execute(terminal::Clear(ClearType::All))?;
    Ok(())
}

pub fn show_error(msg: &str) -> io::Result<()> {
    ceprintln!("<red>{}</>", msg);
    println!("\npress <enter> to quit...");

    // wait for key press
    loop {
        match read()? {
            Event::Key(_) => return Ok(()),
            _ => continue,
        }
    }
}

pub fn show_instructions() {
    print!("{}", INSTRUCTIONS);
}

//////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////

const TITLE_BANNER: &str = "
███╗   ██╗██╗     ██████╗                                    
████╗  ██║██║     ╚════██╗                                   
██╔██╗ ██║██║      █████╔╝                                   
██║╚██╗██║██║     ██╔═══╝                                    
██║ ╚████║███████╗███████╗                                   
╚═╝  ╚═══╝╚══════╝╚══════╝                                   
 ██████╗ ██████╗ ███╗   ██╗████████╗██████╗  ██████╗ ██╗     
██╔════╝██╔═══██╗████╗  ██║╚══██╔══╝██╔══██╗██╔═══██╗██║     
██║     ██║   ██║██╔██╗ ██║   ██║   ██████╔╝██║   ██║██║     
██║     ██║   ██║██║╚██╗██║   ██║   ██╔══██╗██║   ██║██║     
╚██████╗╚██████╔╝██║ ╚████║   ██║   ██║  ██║╚██████╔╝███████╗
 ╚═════╝ ╚═════╝ ╚═╝  ╚═══╝   ╚═╝   ╚═╝  ╚═╝ ╚═════╝ ╚══════╝
██████╗  █████╗ ███╗   ██╗███████╗██╗                        
██╔══██╗██╔══██╗████╗  ██║██╔════╝██║                        
██████╔╝███████║██╔██╗ ██║█████╗  ██║                        
██╔═══╝ ██╔══██║██║╚██╗██║██╔══╝  ██║                        
██║     ██║  ██║██║ ╚████║███████╗███████╗                   
╚═╝     ╚═╝  ╚═╝╚═╝  ╚═══╝╚══════╝╚══════╝                   
                                                             
";

const INSTRUCTIONS: &str = "
    1.      Enter the url above in the browser of the mobile device
            you want to control the coasters from
            (device has to be in the same network as this PC)
    
    2.      On this website, press on the connect button

    3.      If you see the panel, you can start NoLimits / RideSims
            as normal, leaving this application running in the background

    4.      Make sure that you activate the panel with the key in the top left
            and that you set the coasters on 'manual dispatch'

            Have fun :D


";
