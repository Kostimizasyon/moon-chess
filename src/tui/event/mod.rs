use ratatui::crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};

// Try to tokio this at some point?

#[derive(Clone, Copy, Debug)]
pub enum Event {
    // Terminal Tick
    Tick,

    // Key Event
    Key(KeyEvent),

    // Mouse Event
    Mouse(MouseEvent),
    
    // Terminal Resize
    Resize(u16, u16)
}

use mpsc::{Sender, Receiver};
use std::{sync::mpsc, thread, time::{Duration, Instant}};

// Event Handler
#[derive(Debug)]
pub struct EventHandler {

    #[allow(dead_code)]
    sender   : Sender<Event>,
    reciever : Receiver<Event>,
    #[allow(dead_code)]
    handler  : thread::JoinHandle<()>
}

use color_eyre::Result;

impl EventHandler {

    pub fn new(tick_rate : u64) -> Self {

        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, reciever) = mpsc::channel();
        let handler = {

            let sender = sender.clone();
            thread::spawn(move || {

                let mut last_tick = Instant::now();

                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("unable to poll event") {
                        match event::read().expect("unable to read event") {
                            CrosstermEvent::Key(e) => {
                                if e.kind == event::KeyEventKind::Press {
                                    sender.send(Event::Key(e))
                                }
                                else  {
                                    Ok(())
                                }
                            }
                            CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                            CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                            _ => unimplemented!(),
                        }
                        .expect("failed to send event") 
                    }
                    
                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick).expect("failed to send tick");
                        last_tick = Instant::now();
                    }

                }

            })
        };

        Self {
            sender,
            reciever,
            handler
        }

    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub fn next(&self) -> Result<Event> {
        Ok(self.reciever.recv()?)
    }

}


