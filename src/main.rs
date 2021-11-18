trait ISubject<'a, T: IObserver> {
    fn add_observer(&mut self, observer: &'a T);
    fn remove_observer(&mut self, observer: &'a T);
    fn notify_observer(&mut self);
}

struct Subject<'a, T: IObserver> {
    observers: Vec<&'a T>,
}

impl<'a, T: IObserver + PartialEq> Subject<'a, T> {
    fn new() -> Subject<'a, T> {
        Subject {
            observers: Vec::new(),
        }
    }
}

impl<'a, T: IObserver + PartialEq> ISubject<'a, T> for Subject<'a, T> {
    fn add_observer(&mut self, observer: &'a T) {
        self.observers.push(observer);
    }

    fn remove_observer(&mut self, observer: &'a T) {
        let option_idx = self.observers.iter().position(|x| *x == observer);
        match option_idx {
            Some(idx) => {
                self.observers.remove(idx);
            }
            None => {}
        }
    }

    fn notify_observer(&mut self) {
        for observer in self.observers.iter() {
            observer.notify();
        }
    }
}

trait IObserver {
    fn notify(&self);
}

#[derive(PartialEq)]
struct Observer {
    name: String,
}

impl IObserver for Observer {
    fn notify(&self) {
        println!("observer {} got the message", self.name);
    }
}

fn main() {
    let mut subject = Subject::new();
    let observer_x = Observer {
        name: "x".to_string(),
    };
    let observer_y = Observer {
        name: "y".to_string(),
    };

    subject.add_observer(&observer_x);
    subject.add_observer(&observer_y);
    subject.notify_observer();
    println!();

    subject.remove_observer(&observer_x);
    subject.notify_observer();
    println!();

    subject.add_observer(&observer_x);
    subject.notify_observer();
}
