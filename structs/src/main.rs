struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    fn decribe(&self) {
        println!("email: {} | username: {}", self.email, self.username);
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

impl Color {
    fn describe(&self) {
        println!("Color({}, {}, {})", self.0, self.1, self.2)
    }
}

impl Point {
    fn describe(&self) {
        println!("Point({}, {}, {})", self.0, self.1, self.2)
    }
}

fn main() {
    let mut livedivulgators = User {
        email: String::from("live@divulgators.com"),
        username: String::from("botzin"),
        active: true,
        sign_in_count: 3,
    };

    livedivulgators.active = false;

    let efraim = User::build_user(
        String::from("efraim@efraimmarcatto.com"),
        String::from("Efraim"),
    );

    let mut ranshy = User {
        email: String::from("ranshy@gmail.com"),
        username: String::from("ranshy"),
        ..livedivulgators
    };

    efraim.decribe();

    let black = Color(0, 1, 0);
    let origin = Point(2, 0, 1);

    black.describe();
    origin.describe();
}
