use ferrumc_macros::{NBTDeserialize, NBTSerialize};

#[derive(NBTDeserialize, NBTSerialize, Debug, Clone, PartialEq, Default)]
#[nbt(is_root)]
#[nbt(rename = "")]
pub struct FurnaceBlockEntity {
    #[nbt(rename = "Items")]
    pub items: Vec<FurnaceItem>,
    #[nbt(rename = "BurnTime")]
    pub burn_time: i16,
    #[nbt(rename = "CookTime")]
    pub cook_time: i16,
    #[nbt(rename = "CookTimeTotal")]
    pub cook_time_total: i16,
}

#[derive(NBTDeserialize, NBTSerialize, Debug, Clone, PartialEq)]
pub struct FurnaceItem {
    #[nbt(rename = "Slot")]
    pub slot: i8,
    #[nbt(rename = "id")]
    pub id: String,
    #[nbt(rename = "Count")]
    pub count: u8,
}
