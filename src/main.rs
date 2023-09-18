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
        format!("[Vinyl] {} by {}", self.name, self.artist)
    }

    fn play(&self) {
        println!("Playing {} (Side {})", self.display_name(), self.side);
    }
}

struct CompactDisc {
    name: String,
    artist: String,
}

impl Album for CompactDisc {
    fn display_name(&self) -> String {
        format!("[CD] {} by {}", self.name, self.artist)
    }

    fn play(&self) {
        println!("Playing {}", self.display_name());
    }
}

enum Albums {
    Vinyl(Vinyl),
    CompactDisc(CompactDisc),
}

fn main() {
    let lover = Albums::Vinyl(Vinyl {
        name: "Lover".to_string(),
        artist: "Taylor Swift".to_string(),
        side: "A".to_string(),
    });
    let midnights = Albums::CompactDisc(CompactDisc {
        name: "Midnights".to_string(),
        artist: "Taylor Swift".to_string(),
    });

    let collection: Vec<Albums> = vec![lover, midnights];
    for mut album in collection {
        match album {
            Albums::Vinyl(ref mut vinyl) => {
                vinyl.play();
                vinyl.flip();
                vinyl.play();
            }
            Albums::CompactDisc(cd) => {
                cd.play();
            }
        }
    }
}
