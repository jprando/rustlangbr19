mod repository {
    pub trait Entity {
        fn get_id(self: &Self) -> u32;
        fn save(self: &mut Self, callback: fn(Result<&Self, &str>));
    }
}

mod stock {
    #[derive(Debug)]
    pub struct Product {
        id: u32,
        pub name: String
    }

    impl Product {
        pub fn new(name: &str) -> Product{
            Product {
                id: 0, name: String::from(name)
            }
        }
    }

    impl super::repository::Entity for Product {
        fn get_id(self: &Self) -> u32 {
            self.id
        }
        fn save(self: &mut Self, callback: fn(Result<&Self, &str>)) {
            println!("saving product {}", self.name);
            self.id = 123;
            callback(Ok(&self));
        }
    }
}

fn show_saved<T>(entity: Result<&T, &str>)
    where T: std::fmt::Debug + repository::Entity
{
    match entity {
        Ok(value) => {
            println!("ID {:?}", value.get_id());
            println!("{:?}", value)
        },
        Err(msg) => {
            println!("## ERROR {:?}", msg)
        }
    }
}

use repository::Entity;
fn main() {
    const PRODUCT_NAME: &str = "Rust Lang BR Sticker";
    let mut rustlang = stock::Product::new(PRODUCT_NAME);
    rustlang.save(show_saved);

    // let fn1 = show_saved::<stock::Product>;
    // let fn2 = fn1;
    // rustlang.save(fn2);
    // fn1(Ok(&rustlang));
    
    // println!("\nponteiro em memoria");
    // println!("para a funcao main\n{:?}", main as fn())

    // clousure traits
    // let double1 = |x| x * 2;
    // assert_eq!(call_with_one(double1), 2);

    // fn double2(num:usize) -> usize {
    //     num * 2
    // }
    // assert_eq!(call_with_one(double2), 2);

    // #[derive(Debug)]
    // enum Status {	
    //     Value(u32), // sintaxe de inicializador
    //     Stop,
    // }

    // let list_of_statuses: Vec<Status> = (0u32..20)
    // Podemos usar essas funções de inicializador
    // como ponteiros de função
    //     .map(Status::Value).collect();
    //     println!("{:#?}", list_of_statuses)
}

// fn call_with_one<F>(func: F) -> usize
//     where F: Fn(usize) -> usize {
//     func(1)
// }

