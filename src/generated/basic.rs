// This is where your template will be created.
use texcore::template::*;
use texcore::*;

// Name of the template
static NAME: &str = "basic";
// Description of the template
static DESC: &str = r#"A Basic LaTeX Document"#;

// Used to distribute the name
pub fn name() -> String {
    NAME.to_string()
}

// where all the template elements will be created
fn elements() -> Vec<Element<Any>> {
    let pagenumbering = Custom::new(r"\pagenumbering{arabic}", Level::Document);
    let newpage = Custom::new(r"\newpage", Level::Document);
    let amsmath = Package::new("amsmath");
    // This macro converts all TexCore types that implement
    // the `Tex` trait to Element<Any>
    Elements![pagenumbering, newpage, amsmath]
}

// where you may declare the version of the template
fn version() -> Version {
    let mut version = Version::default();
    // semantic versioning follows M.m.p
    // where M is major, m is minor and p is patch
    let major = 1;
    let minor = 0;
    let patch = 0;
    // sets the version
    version.set_version(major, minor, patch);
    version
}

// generates the template
pub async fn generate_template() -> Template {
    // sets the metadata defaults of the template
    let mut metadata = Metadata::default();
    metadata.maketitle = true;
    // we will create a new template setting the name, description and metadata
    let mut template = Template::new(NAME, DESC, &metadata);
    // sets the template's version
    template.version = version();
    // push the elements into the template
    template.push_element_array(elements()).await;
    template
}
