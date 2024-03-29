use crate::Terminal;
use termion::event::Key;



pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self){
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit{
                break;
            }
            if let Err(error) = self.process_keypress(){
                die(error);
            }
        }
    }
    pub fn default() ->Self {
        Self { 
            should_quit : false,
            terminal: Terminal::default().expect("Faild to initialize terminal"), 
        }
    }
    
    fn refresh_screen(&self) -> Result<(), std::io::Error>{
        Terminal::cursor_hide();
        
        Terminal::cursor_position(0,0);
        if self.should_quit{
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0,0);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(),std::io::Error>{
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit =true,
            _=>(),
        }
        Ok(())
    }

    fn draw_rows(&self){
        for _ in 0..self.terminal.size().height - 1 {
            Terminal::clear_current_line();
            println!("~\r");
        }
    }
    
}


fn die(e: std::io::Error){
    panic!("{}",e);
}