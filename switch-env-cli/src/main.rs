use anyhow::Result;
use switch_env_cli::{make_data, render_html, search_scss};

fn main() -> Result<()> {
    let scss_file_list = search_scss()?;
    let data = make_data(scss_file_list);
    render_html(data)?;
    Ok(())
}
