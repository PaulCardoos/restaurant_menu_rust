#[derive(Default, Debug, Clone)]
pub struct Item {
    name : String,
    price : f32,
    description : Option<String>, 
}

impl Item {
    pub fn new(item_name:&str, item_price:f32) -> Item {
        let i = Item{
            name : item_name.to_string(),
            price : item_price, 
            description : None,
        };
        i
        
    }

    pub fn item_description(mut self, description:&str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn build(self) -> Self{
        Item {
            name : self.name, 
            price : self.price,
            description : self.description,
        }
    }

    pub fn print_item(&self){
        println!("Name : {0:<20} Price : {1}", self.name, self.price)
    }
}