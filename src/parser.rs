use pulldown_cmark;
use std::{fs, io, path};

pub fn generate_static_pages(
    markdown_dir: &path::Path,
    html_dir: &path::Path,
) -> Result<Vec<Box<path::Path>>, io::Error> {
    fs::create_dir_all(markdown_dir)?;
    fs::create_dir_all(html_dir)?;
    let md_files = fs::read_dir(markdown_dir)?
        .flat_map(|res| {
            res.map(|e| e.path().into_boxed_path())
                .into_iter()
                .filter(|e| {
                    e.extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
                })
        })
        .collect::<Vec<_>>();
    let html_files = md_files
        .iter()
        .filter_map(|f| {
            f.with_extension("html")
                .file_name()
                .map(|f| f.to_string_lossy().into_owned())
        })
        .map(|f| html_dir.join(f).into_boxed_path())
        .collect::<Vec<_>>();

    // Propagate first error back to the caller if there is one
    let results = md_files
        .iter()
        .zip(html_files.iter())
        .map(|(md_file, html_file)| generate_html_from_markdown(md_file, html_file))
        .collect::<Result<Vec<_>, _>>();

    match results {
        Ok(_) => Ok(html_files),
        Err(e) => Err(e),
    }
}

pub fn generate_html_from_markdown(
    markdown_file: &path::Path,
    html_file: &path::Path,
) -> Result<(), io::Error> {
    let contents = fs::read_to_string(markdown_file)?;
    let parser = pulldown_cmark::Parser::new(&contents);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    fs::write(html_file, html_output)?;
    Ok(())
}
