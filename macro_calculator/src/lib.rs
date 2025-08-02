use json::{object, JsonValue};
 
#[derive( Clone, Debug)]
pub struct Food {
    // expected public fields 
    pub name : String,
    pub calories: (String, String), 
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    if foods.len() == 0 {
         return JsonValue::new_object();
    }
    let food = foods[0].clone();

    let mut cals : f64 = 0.0;
    let mut carbs : f64 = 0.0;
    let mut proteins : f64 = 0.0;
    let mut fats : f64 = 0.0;

    for food in foods.iter() {
        cals += food.calories.1.replace("kcal", "").parse::<f64>().unwrap() * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;// * food.nbr_of_portions;
        proteins += food.proteins * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;
    }

    let jsoned = object!{
        cals : (cals * 100.0).round() / 100.0, // 100.0,
        carbs : (carbs * 100.0).round() / 100.0, // / 100.0,
        proteins : (proteins * 100.0).round() / 100.0, //, //.round() / 100.0, 
        fats: (fats * 100.0) .round() / 100.0 ,// / 100., 
    };

    jsoned
}

