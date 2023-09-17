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
        println!("[Vinyl] {} (Side {})", self.display_name(), self.side);
    }
}

fn main() {
    let lover = Vinyl {
        name: "Lover".to_string(),
        artist: "Taylor Swift".to_string(),
        side: "A".to_string(),
    };
    let collection = vec![lover];
    for mut album in collection {
        album.play();
        album.flip();
        album.play();
    }
}
