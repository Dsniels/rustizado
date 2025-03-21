enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        }
        return false;
    }

    fn is_green_parts(&self) -> bool {
        match self {
            Color::Red => return false,
            Color::Green => return false,
            Color::Blue => return true,
        }
    }
}

fn print_color(color: Color) -> () {
    match color {
        Color::Red => println!("red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}

#[derive(Debug)]
struct Custom {
    age: usize,
    name: String,
}

#[derive(Debug)]
enum Item {
    Number(usize),
    String(String),
    MyCustom(Custom),
}

fn append(items: &mut Vec<Item>) {
    items.push(Item::String("Hola".into()));

    println!("{:?}", *items);
    println!("{:?}", &items);
    println!("{:?}", items);
}

fn practice(nums: &Vec<usize>, index: usize) -> usize {
    return nums.get(index).unwrap_or(&index) * 5;
}

fn multiply(num: Option<usize>) -> Option<usize> {
    //return num.unwrap_or(0) * 5;

    return Some(num?);
}

fn read_file(filename: String) {
    let file = std::fs::read_to_string(filename).expect("Unable to read the file to string");
    file.lines().for_each(|line| {
        if let Ok(value) = line.parse::<usize>() {
            println!("{}", value);
        } else {
            println!("Line not a number");
        }
    });
}
fn main() {
    //let ooo = Color::Green;
    //
    let mut items: Vec<Item> = vec![];
    println!("{:?}", items);

    append(&mut items);
    let arg = std::env::args()
        .nth(1)
        .expect("The filename to be passed in");
    read_file(arg);
}
