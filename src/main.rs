trait Album {
    fn display_name(&self) -> String;
    fn play(&self);
}

struct Vinyl {
    name: String,
    artist: String,
    side: String,
}

impl Vinyl {
    fn flip(&mut self) {
        if self.side == "A" {
            self.side = "B".to_string();
        } else {
            self.side = "A".to_string();
        }
    }
}

impl Album for Vinyl {
    fn display_name(&self) -> String {
        format!("{} by {}", self.name, self.artist)
    }

    fn play(&self) {
        println!(
            "[Vinyl] Playing {} (Side {})",
            self.display_name(),
            self.side
        );
    }
}

struct CompactDisc {
    name: String,
    artist: String,
}

impl Album for CompactDisc {
    fn display_name(&self) -> String {
        format!("{} by {}", self.name, self.artist)
    }

    fn play(&self) {
        println!("[CD] Playing {}", self.display_name());
    }
}

fn main() {
    let lover = Box::new(Vinyl {
        name: "Lover".to_string(),
        artist: "Taylor Swift".to_string(),
        side: "A".to_string(),
    }) as Box<dyn Album>;
    let midnights = Box::new(CompactDisc {
        name: "Midnights".to_string(),
        artist: "Taylor Swift".to_string(),
    }) as Box<dyn Album>;

    let collection: Vec<Box<dyn Album>> = vec![lover, midnights];
    for mut album in collection {
        album.play();

        // Trying to know whether it is a vinyl or not, to know if we need to
        // flip it, fails with:
        //
        // error[E0599]: no method named `downcast` found for struct `Box<dyn Album>` in the current scope
        //    --> src/main.rs:79:34
        //    |
        // 79 |         if let Ok(vinyl) = album.downcast::<Vinyl>() {
        //    |                                  ^^^^^^^^ method not found in `Box<dyn Album>`
        //    |
        //    = note: the method was found for
        //            - `Box<(dyn Any + 'static), A>`
        //            - `Box<(dyn Any + Send + 'static), A>`
        //            - `Box<(dyn Any + Send + Sync + 'static), A>`
        if let Ok(vinyl) = album.downcast::<Vinyl>() {
            vinyl.flip();
            vinyl.play();
        }
    }
}
