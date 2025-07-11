use std::fmt;

#[derive(Debug, Clone)]
pub enum DeviceType {
    Stove,
}

#[derive(Debug, Clone)]
pub enum NPCType {
    Customer,
}

#[derive(Debug, Clone)]
pub enum InteractableType {
    NPC(NPCType),
    Device(DeviceType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ItemID {
    #[default]
    None,

    // Raw Ingredients
    RawMeat,
    Fish,
    Herb,
    Egg,
    Milk,
    Flour,
    Sugar,
    Salt,
    Butter,
    Cheese,
    Tomato,
    Onion,
    Garlic,
    Potato,
    Carrot,
    Mushroom,
    Pepper,
    Cabbage,
    Corn,
    Wheat,
    Rice,
    Apple,
    Berry,
    Lemon,
    Honey,
    Oil,

    // Prepared or Cooked Items
    CookedMeat,
    GrilledFish,
    VegetableSoup,
    Bread,
    FriedEgg,
    Omelette,
    CheeseToast,
    MeatStew,
    FishStew,
    Salad,
    RoastedVegetables,
    FruitPie,
    BerryJam,
    Pancakes,
    Cake,
    Cookies,
    GrilledCheese,
    RiceBowl,
    StirFry,
    Smoothie,
    HerbTea,
}

impl fmt::Display for ItemID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            ItemID::None => "None",

            // Raw Ingredients
            ItemID::RawMeat => "Raw Meat",
            ItemID::Fish => "Fish",
            ItemID::Herb => "Herb",
            ItemID::Egg => "Egg",
            ItemID::Milk => "Milk",
            ItemID::Flour => "Flour",
            ItemID::Sugar => "Sugar",
            ItemID::Salt => "Salt",
            ItemID::Butter => "Butter",
            ItemID::Cheese => "Cheese",
            ItemID::Tomato => "Tomato",
            ItemID::Onion => "Onion",
            ItemID::Garlic => "Garlic",
            ItemID::Potato => "Potato",
            ItemID::Carrot => "Carrot",
            ItemID::Mushroom => "Mushroom",
            ItemID::Pepper => "Pepper",
            ItemID::Cabbage => "Cabbage",
            ItemID::Corn => "Corn",
            ItemID::Wheat => "Wheat",
            ItemID::Rice => "Rice",
            ItemID::Apple => "Apple",
            ItemID::Berry => "Berry",
            ItemID::Lemon => "Lemon",
            ItemID::Honey => "Honey",
            ItemID::Oil => "Oil",

            // Prepared or Cooked Items
            ItemID::CookedMeat => "Cooked Meat",
            ItemID::GrilledFish => "Grilled Fish",
            ItemID::VegetableSoup => "Vegetable Soup",
            ItemID::Bread => "Bread",
            ItemID::FriedEgg => "Fried Egg",
            ItemID::Omelette => "Omelette",
            ItemID::CheeseToast => "Cheese Toast",
            ItemID::MeatStew => "Meat Stew",
            ItemID::FishStew => "Fish Stew",
            ItemID::Salad => "Salad",
            ItemID::RoastedVegetables => "Roasted Vegetables",
            ItemID::FruitPie => "Fruit Pie",
            ItemID::BerryJam => "Berry Jam",
            ItemID::Pancakes => "Pancakes",
            ItemID::Cake => "Cake",
            ItemID::Cookies => "Cookies",
            ItemID::GrilledCheese => "Grilled Cheese",
            ItemID::RiceBowl => "Rice Bowl",
            ItemID::StirFry => "Stir Fry",
            ItemID::Smoothie => "Smoothie",
            ItemID::HerbTea => "Herb Tea",
        };
        write!(f, "{}", text)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecipeID {
    CookedMeat,
    VegetableSoup,
    GrilledFish,
    FishStew,
}

pub enum InventoryAddReason {
    None,
    Crafted,
}
