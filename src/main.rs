#[derive(Debug)]
enum Gender {
    Men,
    Women,
}

#[derive(Debug)]
struct Shoe {
    name: String,
    gender: Gender,
    size: String,
}

#[derive(Debug)]
struct Apparel {
    name: String,
    gender: Gender,
    size: String,
    material: String,
}

#[derive(Debug)]
enum Product {
    Shoe(Shoe),
    Apparel(Apparel),
    Test,
}

impl Product {
    fn name(&self) -> &str {
        match self {
            Product::Shoe(shoe) => &shoe.name[..],
            Product::Apparel(apparel) => &apparel.name[..],
            _ => "Test",
        }
    }

    fn gender(&self) -> Option<&Gender> {
        match self {
            Product::Shoe(shoe) => Some(&shoe.gender),
            Product::Apparel(apparel) => Some(&apparel.gender),
            _ => None,
        }
    }

    fn size(&self) -> Option<&str> {
        match self {
            Product::Shoe(shoe) => Some(&shoe.size[..]),
            Product::Apparel(apparel) => Some(&apparel.size[..]),
            _ => None,
        }
    }

    fn material(&self) -> Option<&str> {
        match self {
            Product::Shoe(_shoe) => None,
            Product::Apparel(apparel) => Some(&apparel.material[..]),
            _ => None,
        }
    }
}

fn main() {
    let shoe = Product::Shoe(Shoe {
        name: String::from("test shoe"),
        gender: Gender::Men,
        size: String::from("M"),
    });

    let apparel = Product::Apparel(Apparel {
        name: String::from("test apparel"),
        gender: Gender::Women,
        size: String::from("L"),
        material: String::from("Cotton"),
    });

    let test = Product::Test;

    dbg!(shoe.name());
    dbg!(shoe.gender());
    dbg!(shoe.size());
    dbg!(shoe.material());

    dbg!(apparel.name());
    dbg!(apparel.gender());
    dbg!(apparel.size());
    dbg!(apparel.material());

    dbg!(test.name());
    dbg!(test.gender());
    dbg!(test.size());
    dbg!(test.material());
}
