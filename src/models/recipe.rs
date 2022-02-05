#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(
    rename_all(deserialize = "camelCase", serialize = "snake_case"),
    untagged
)]
pub enum Recipe {
    Shaped(ShapedRecipe),
    Shapeless(ShapelessRecipe),
}

impl Default for Recipe {
    fn default() -> Self {
        Self::Shaped(Default::default())
    }
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct ShapedRecipe {
    result: RecipeItem,
    in_shape: Shape,
    out_shape: Option<Shape>,
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct ShapelessRecipe {
    result: RecipeItem,
    ingredients: Vec<RecipeItem>,
}

pub type Shape = Vec<Vec<RecipeItem>>;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(
    rename_all(deserialize = "camelCase", serialize = "snake_case"),
    untagged
)]
pub enum RecipeItem {
    ID(u32),
    IDMetadataArray([u32; 2]),
    IDMetadataCountObject(IDMetadataCountObject),
    Null(Option<()>),
}

impl Default for RecipeItem {
    fn default() -> Self {
        Self::Null(None)
    }
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct IDMetadataCountObject {
    id: i32,
    metadata: Option<i32>,
    count: Option<u32>,
}
