mod front_of_houses;

pub use crate::front_of_house::hosting;
use crate::front_of_houses::{hosting, serving}

pub fn eat_at_restaurant(){

    // absoluter Pfad ohne use crate::front_of_house::hosting;
    // crate::front_of_house::hosting::add_to_waitlist();

    // relativer Pfad ohne use crate::front_of_house::hosting;
    // front_of_house::hosting::add_to_waitlist();

    // mit use crate::front_of_house::hosting;
    hosting::add_to_waitlist();
    hosting::seat_at_table();
    hosting::take_order();();
    hosting::server_order();
    hosting::take_paymennt();


    // Bestelle im Sommer ein Frühstück mit Roggentoast
    let mut meal = back_of_house::Breakfast::summer("Roggen");
    // Ändere unsere Meinung darüber, welche Brotsorte wir gerne hätten
    meal.toast = String::from("Weizen");
    println!("Ich möchte {}-Toast", meal.toast);

    // Die nächste Zeile lässt sich nicht kompilieren, wenn wir sie nicht
    // auskommentieren; wir dürfen die Früchte der Saison, die wir mit der
    // Mahlzeit bekommen, weder sehen noch verändern.
    // meal.seasonal_fruit = String::from("Heidelbeeren");


}


mod back_of_house{

    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit: String,
    } 

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast),
                seasonal_fruit: String::from("Pfirsiche"), 
            }
        }
    }

}