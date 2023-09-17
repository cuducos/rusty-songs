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
    let lover = Vinyl {
        name: "Lover".to_string(),
        artist: "Taylor Swift".to_string(),
        side: "A".to_string(),
    };
    let midnights = CompactDisc {
        name: "Midnights".to_string(),
        artist: "Taylor Swift".to_string(),
    };

    let vinyl_collection = vec![lover];
    for mut album in vinyl_collection {
        album.play();
        album.flip();
        album.play();
    }

    let cd_collection = vec![midnights];
    for album in cd_collection {
        album.play();
    }
}
