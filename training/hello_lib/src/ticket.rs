use tokio::sync::{Semaphore, SemaphorePermit};

pub struct Meseum {
    remaining_tickets: Semaphore,
}

impl Meseum {
    pub fn new(total: usize) -> Self {
        Self {
            remaining_tickets: Semaphore::new(total),
        }
    }

    pub fn get_ticket(&self) -> Option<Ticket> {
        match self.remaining_tickets.try_acquire() {
            Ok(permit) => Some(Ticket::new(permit)),
            Err(_) => None,
        }
    }

    pub fn tickets(&self) -> usize {
        self.remaining_tickets.available_permits()
    }
}

#[derive(Debug)]
pub struct Ticket<'a> {
    permit: SemaphorePermit<'a>,
}

impl<'a> Drop for Ticket<'a> {
    fn drop(&mut self) {
        println!("ticket freed");
    }
}

impl<'a> Ticket<'a> {
    pub fn new(permit: SemaphorePermit<'a>) -> Self {
        Self { permit }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let meseum = Meseum::new(10);
        let ticket = meseum.get_ticket().unwrap();
        println!("{:?}", ticket);
        assert_eq!(meseum.tickets(), 9);
        let _tickets: Vec<_> = (0..9).map(|_| meseum.get_ticket().unwrap()).collect();
        assert_eq!(meseum.tickets(), 0);
        assert!(meseum.get_ticket().is_none());
        drop(ticket);
        println!("------------------");
        assert!(meseum.get_ticket().is_some());
        println!("------------------");
    }
}
