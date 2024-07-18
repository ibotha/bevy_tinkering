use bevy::{
    asset::{io::Reader, AssetLoader, AssetPath, AsyncReadExt, LoadContext},
    prelude::*,
};

#[derive(Default, Clone)]
pub(super) struct TextureAtlasLayoutLoader;

#[derive(Asset, Reflect)]
pub struct TextureAndLayout {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

impl AssetLoader for TextureAtlasLayoutLoader {
    type Asset = TextureAndLayout;
    type Settings = ();
    type Error = std::io::Error;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut string = String::new();
        reader.read_to_string(&mut string).await?;

        let lines: Vec<&str> = string.lines().collect();
        let image_name = lines[0];
        let base_path = load_context
            .asset_path()
            .parent()
            .unwrap_or(AssetPath::default());
        let image_asset = load_context
            .loader()
            .direct()
            .load::<Image>(format!("{base_path}/{image_name}"))
            .await
            .map_err(|e| Self::Error::new(std::io::ErrorKind::Other, e))?;
        let image = image_asset.get();

        let mut atlas = TextureAtlasLayout::new_empty(UVec2::new(image.width(), image.height()));

        for line in &lines[1..] {
            let parts = line.split(" ").collect::<Vec<&str>>();

            let (name, x, y, width, height) = (
                parts[0],
                parts[1].parse::<u32>().expect("x is a number"),
                parts[2].parse::<u32>().expect("y is a number"),
                parts[3].parse::<u32>().expect("width is a number"),
                parts[4].parse::<u32>().expect("height is a number"),
            );

            info!("Loaded {name}, {x:?} {y:?} {width:?} {height:?}");
            let (max_x, max_y) = (x + width, y + height);
            if max_x > atlas.size.x {
                atlas.size.x = max_x;
            }
            if max_y > atlas.size.y {
                atlas.size.y = max_y;
            }
            atlas.add_texture(URect::new(x, y, x + width, y + height));
        }

        let path = load_context.asset_path();
        Ok(TextureAndLayout {
            layout: load_context.add_labeled_asset(format!("{path}:texture").to_string(), atlas),
            texture: load_context.load::<Image>(format!("{base_path}/{image_name}")),
        })
    }

    fn extensions(&self) -> &[&str] {
        &["atl"]
    }
}
