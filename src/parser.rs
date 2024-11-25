use pulldown_cmark;
use std::{fs, io, path};

pub fn generate_static_pages(
    markdown_dir: &path::Path,
    html_dir: &path::Path,
) -> Result<Vec<Box<path::Path>>, io::Error> {
    fs::create_dir_all(html_dir)?; // Create the HTML directory

    let mut html_files = Vec::new();

    // Function to recursively process directories
    fn process_directory(
        markdown_dir: &path::Path,
        html_dir: &path::Path,
        html_files: &mut Vec<Box<path::Path>>,
    ) -> Result<(), io::Error> {
        for entry in fs::read_dir(markdown_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let new_html_dir = html_dir.join(path.file_name().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::Other, "Failed to get file name")
                })?);
                fs::create_dir_all(&new_html_dir)?;

                process_directory(&path, &new_html_dir, html_files)?;
            } else if let Some(ext) = path.extension() {
                // Parse any markdown files into html
                if ext.eq_ignore_ascii_case("md") {
                    let html_file_name = path
                        .with_extension("html")
                        .file_name()
                        .ok_or_else(|| {
                            io::Error::new(io::ErrorKind::Other, "Failed to get file name")
                        })?
                        .to_string_lossy()
                        .into_owned();

                    let new_html_file_path = html_dir.join(&html_file_name);
                    html_files.push(new_html_file_path.clone().into_boxed_path());

                    generate_html_from_markdown(&path, &new_html_file_path)?;
                } else {
                    // Copy over all non-markdown files directly
                    let basename = path.file_name().ok_or_else(|| {
                        io::Error::new(io::ErrorKind::Other, "Failed to get file name")
                    })?;
                    let target = html_dir.join(basename);
                    fs::copy(&path, &target)?;
                }
            }
        }
        Ok(())
    }

    // Start processing the main markdown directory
    process_directory(markdown_dir, html_dir, &mut html_files)?;

    Ok(html_files)
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
