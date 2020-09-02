use crate::{
    components::SmmdbCoursePanel,
    widgets::{SaveWidget, SmmdbWidget},
    AppState,
};

use anyhow::Result;
use iced::{Element, Row};
use indexmap::IndexMap;
use std::path::PathBuf;

pub struct SavePage {
    save: smmdb_lib::Save,
    location: PathBuf,
    save_widget: SaveWidget,
    smmdb_widget: SmmdbWidget,
}

impl SavePage {
    pub fn new(save: smmdb_lib::Save, location: PathBuf) -> SavePage {
        SavePage {
            save_widget: SaveWidget::new(&save),
            save,
            location,
            smmdb_widget: SmmdbWidget::new(),
        }
    }

    pub fn view<'a>(
        &'a mut self,
        state: &AppState,
        smmdb_course_panels: &'a mut IndexMap<String, SmmdbCoursePanel>,
    ) -> Element<crate::Message> {
        Row::new()
            .push(self.save_widget.view(state, &self.location))
            .push(self.smmdb_widget.view(state, smmdb_course_panels))
            .into()
    }

    pub async fn swap_courses(&mut self, first: u8, second: u8) -> Result<()> {
        self.save.swap_course(first, second)?;
        self.save
            .save()
            .await
            .map_err(|err| -> anyhow::Error { err.into() })?;
        self.generate_course_panels();
        Ok(())
    }

    pub async fn add_course(&mut self, index: u8, course: smmdb_lib::Course2) -> Result<()> {
        self.save.add_course(index, course)?;
        self.save
            .save()
            .await
            .map_err(|err| -> anyhow::Error { err.into() })?;
        self.generate_course_panels();
        Ok(())
    }

    fn generate_course_panels(&mut self) {
        self.save_widget.generate_course_panels(&self.save);
    }
}
