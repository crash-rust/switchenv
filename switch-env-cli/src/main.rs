use anyhow::Result;
use glob::glob;
use handlebars::{to_json, Handlebars};
use serde_json::{Map, Value as Json};
use std::{fs::File, path::PathBuf};

pub fn main() -> Result<()> {
    let scss_file_list = search_scss()?;
    let data = make_data(scss_file_list);
    render_html(data)?;
    Ok(())
}

fn search_scss() -> Result<Vec<PathBuf>> {
    let mut scss_file_list: Vec<PathBuf> = Vec::new();
    for entry in glob("switch-env-ui/src/**/*.scss")? {
        match entry {
            Ok(path) => {
                let path = path.strip_prefix("switch-env-ui/")?;
                // println!("path {:?}", path);
                scss_file_list.push(path.into());
            }
            Err(e) => return Err(e.into()),
        }
    }
    Ok(scss_file_list)
}

fn make_data(scss_file_list: Vec<PathBuf>) -> Map<String, Json> {
    let mut data: Map<String, Json> = Map::new();
    data.insert("link_tags".to_string(), to_json(&scss_file_list));
    data
}

fn render_html(data: Map<String, Json>) -> Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("template", "switch-env-cli/src/template.hbs")?;

    let mut output_file = File::create("switch-env-ui/index.html")?;

    handlebars.render_to_write("template", &data, &mut output_file)?;

    Ok(())
}
