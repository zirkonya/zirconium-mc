use zr_protocol::macros::Serialize;

#[derive(Serialize, Debug)]
pub struct RecipeBookSettings {
    pub crafting_recipe_book_open: bool,
    pub crafting_recipe_filter_active: bool,
    pub smelting_recipe_book_open: bool,
    pub smelting_recipe_filter_active: bool,
    pub blast_furnace_recipe_book_open: bool,
    pub blast_furnace_recipe_filter_active: bool,
    pub smoker_recipe_book_open: bool,
    pub smoker_recipe_filter_active: bool,
}
