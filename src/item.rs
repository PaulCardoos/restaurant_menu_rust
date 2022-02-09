#[derive(Debug)]
pub struct Item {
    name : String,
    price : f32,
    description : String, 
}

impl Item {
    pub fn new() -> Self {
        let i =  Item{
            name : String::from(""),
            price : 0.00, 
            description : String::from(""),
        };
        i
    }

    pub fn item_name(mut self, name:String) -> Self {
        self.name = name;
        self
        
    }

    pub fn item_price(mut self, price:f32) -> Self {
        self.price = price;
        self
    }

    pub fn item_description(mut self, description:String) -> Self {
        self.description = description;
        self
    }

    pub fn print_item(&self){
        println!("Name : {0:<20} Price : {1}", self.name, self.price)
    }
}