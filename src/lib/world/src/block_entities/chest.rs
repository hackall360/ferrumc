use ferrumc_macros::{NBTDeserialize, NBTSerialize};

#[derive(NBTDeserialize, NBTSerialize, Debug, Clone, PartialEq, Default)]
#[nbt(is_root)]
#[nbt(rename = "")]
pub struct ChestBlockEntity {
    #[nbt(rename = "Items")]
    pub items: Vec<ChestItem>,
}

#[derive(NBTDeserialize, NBTSerialize, Debug, Clone, PartialEq)]
pub struct ChestItem {
    #[nbt(rename = "Slot")]
    pub slot: i8,
    #[nbt(rename = "id")]
    pub id: String,
    #[nbt(rename = "Count")]
    pub count: u8,
}
