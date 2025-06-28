#[derive(Debug)]
pub enum DeviceType {
    Stove,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemID {
    RawMeat,
    Herb,
    Fish,
    CookedMeat,
    VegetableSoup,
    GrilledFish,
}
