use std::path::Path;
use std::fs::File;
use std::io::Write;

use clap::Parser;

use downloader::Downloader;

mod observable_json;
use observable_json::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

  /// an Observable notebook short reference ("@hrbrmstr/just-the-facts") or a full URL 
  #[clap(long)]
  ohq_ref: String,

  /// directory to place Quarto project and files (will be created if it does not exist)
  #[clap(long)]
  output_dir: String,

  /// optional filename for the main Quarto document (will be taken from the slug in `ohq_ref`; e.g. "just-the-facts" from the example param)
  #[clap(long)]
  filename: Option<String>,

  /// turn cell echo on in the Quarto document (default is to not echo)
  #[clap(long)]
  echo: bool,

  /// Print Notebook metadata during processing
  #[clap(long)]
  verbose: bool,
  
}

fn main() {

  // ensure we have arguments
  let args = Args::parse();

  // make the speficied directory
  let output_dir = shellexpand::full(&args.output_dir).expect("Cannot parse/expand the provided directory path.");
  std::fs::create_dir_all(output_dir.to_string()).expect("Cannot create the speficied output directory.");

  // build the obshq URL if needed
  let obs_url = if args.ohq_ref.starts_with("@") { format!("https://observablehq.com/{}", args.ohq_ref) } else { args.ohq_ref };
  
  // start the scraper
  let client = reqwest::blocking::Client::new();
  let response = client
    .get(obs_url.clone())
    .send().expect("Error retrieving Notebook.")
    .text().expect("Error extracting body from GET request response.");

    let document = scraper::Html::parse_document(&response);

    // this is where they hide the actual notebook code
    let cell_selector = scraper::Selector::parse("script#__NEXT_DATA__").unwrap();

    let mut cells = document.select(&cell_selector).map(|x| x.inner_html());
    let cell = cells.next().expect("No Observable data block found.");

    // did they give us a filename?
    let filename_option = args.filename;

    // I likely need to trim down this and setup a separate job to monitor the schema
    let model: ObservableData = serde_json::from_str(&cell)
    .expect("It appears the JSON notebook schema has changed; please file an issue @ https://github.com/hrbrmstr/ohq2quarto/issues");

    // TODO there is a plethora of metadata here that we can shove into the YAML
    let nb = model.props.page_props.initial_notebook; 
    let nodes = nb.nodes;
    let title = nb.title;

    let authors: Vec<String> = nb.authors.iter()
    .map(|author| author.name.clone())
    .collect();

    if args.verbose {
      println!("      Title: {}", title);
      println!("       Slug: {}", nb.slug);
      println!("  Author(s): {}", authors.join(","));
      if nb.description != "" {
        println!("Description: {}", nb.description);
      }
      println!("  Copyright: {}", nb.copyright);
      if let Some(l) = nb.license {
        println!("    License: {}", l);
      }
      println!(" Observable: {}", obs_url);
    }

    // make a filename from the notebook slug if the caller did not specify one
    let qmd_file: String = if let Some(x) = &filename_option {
      x.clone()
    } else {
      format!("{}.qmd", nb.slug)
    };

    // make _quarto.yml
    let file_path = Path::new(output_dir.as_ref()).join("_quarto.yml");
    let mut prj = File::create(file_path).expect("Error opening _quarto.yml file for writing.");
    writeln!(prj, "project:").unwrap();
    writeln!(prj, "  title: {}", title).unwrap();
    
    // make the qmd file
    let file_path = Path::new(output_dir.as_ref()).join(qmd_file);
    let mut qmd = File::create(file_path).expect("Error opening qmd file for writing.");

    writeln!(qmd, "---").unwrap();
    writeln!(qmd, "title: '{}'", title).unwrap();
    writeln!(qmd, "author: '{}'", authors.join(",")).unwrap();
    writeln!(qmd, "format: html").unwrap();
    writeln!(qmd, "echo: {}", args.echo).unwrap(); 
    writeln!(qmd, "observable: '{}'", obs_url).unwrap();
    writeln!(qmd, "---").unwrap();
    writeln!(qmd).unwrap();

    for node in nodes {
      writeln!(qmd, "```{{ojs}}").unwrap();
      match node.mode.as_str() {
        "md" => writeln!(qmd, "md`{}`", node.value).unwrap(),
        "html" => writeln!(qmd, "html`{}`", node.value).unwrap(),
        _ => writeln!(qmd, "{}", node.value).unwrap()
      };
      writeln!(qmd, "```").unwrap();
      writeln!(qmd).unwrap();
    }

    // Check to see if we need to download any FileAttachments
    if let Some(files) = nb.files {

      let output_dir = Path::new(output_dir.as_ref());

      let mut downloader = Downloader::builder()
        .download_folder(output_dir)
        .parallel_requests(1)
        .build()
        .expect("Error setting up file downloader");

      for file in files {

        let download_file = downloader::Download::new(file.url.as_ref())
          .file_name(file.name.as_ref());

        downloader.download(&[download_file])
        .expect("Error downloading one of the FileAttachments");

      };

    };
    
}
