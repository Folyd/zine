use anyhow::Result;
use std::path::Path;
use tera::Context;

mod article;
mod end_matter;
mod page;
mod season;
mod site;
mod theme;
mod zine;

pub use self::zine::Zine;
pub use article::Article;
pub use end_matter::EndMatter;
pub use page::Page;
pub use season::Season;
pub use site::Site;
pub use theme::Theme;

/// A trait represents the entity of zine config file.
///
/// A zine entity contains two stage:
/// - **parse**, the stage the entity to parse its attribute, such as parse markdown to html.
/// - **render**, the stage to render the entity to html file.
///
/// [`Entity`] have default empty implementations for both methods.
#[allow(unused_variables)]
pub trait Entity {
    fn parse(&mut self, source: &Path) -> Result<()> {
        Ok(())
    }

    fn render(&self, context: Context, dest: &Path) -> Result<()> {
        Ok(())
    }
}

impl<T: Entity> Entity for Option<T> {
    fn parse(&mut self, source: &Path) -> Result<()> {
        if let Some(entity) = self {
            entity.parse(source)?;
        }
        Ok(())
    }

    fn render(&self, context: Context, dest: &Path) -> Result<()> {
        if let Some(entity) = self {
            entity.render(context, dest)?;
        }
        Ok(())
    }
}

impl<T: Entity> Entity for Vec<T> {
    fn parse(&mut self, source: &Path) -> Result<()> {
        for item in self {
            item.parse(source)?;
        }
        Ok(())
    }

    fn render(&self, render: Context, dest: &Path) -> Result<()> {
        for item in self {
            item.render(render.clone(), dest)?;
        }
        Ok(())
    }
}
