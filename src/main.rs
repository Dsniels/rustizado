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

    age : usize,
    name : String
}

#[derive(Debug)]
enum Item {
    Number(usize),
    String(String),
    MyCustom(Custom),
    
}


fn append(items : &mut Vec<Item>){
    
    items.push(Item::String("Hola".into()));

    println!("{:?}", *items);
    println!("{:?}", &items);
    println!("{:?}", items);
}



fn main() {
    //let ooo = Color::Green;
    //
    let mut items : Vec<Item> = vec![];
    println!("{:?}", items);

    append(&mut items);



    //println!("{:?}", ooo.is_green());

}







