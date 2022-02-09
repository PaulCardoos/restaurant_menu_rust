mod item;

fn main() {
    let baba = item::Item::new().item_name(String::from("Hummes"))
    .item_price(5.00)
    .item_description(String::from("Chickpeas are the shizzz"));

    
    baba.print_item();
    

}
