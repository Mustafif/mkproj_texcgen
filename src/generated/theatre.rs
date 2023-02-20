// This is where your template will be created.
use texcore::template::*;
use texcore::TextType::Normal;
use texcore::*;

// Name of the template
static NAME: &str = "theatre";
// Description of the template
static DESC: &str = r#"Create your own script for a play"#;

// Used to distribute the name
pub fn name() -> String {
    NAME.to_string()
}

// where all the template elements will be created
fn elements() -> Vec<Element<Any>> {
    let toc = Custom::new(r"\tableofcontents", Level::Document);
    let newpage = Custom::new(r"\newpage", Level::Document);
    let character = Custom::new(r"\Character[Description]{Name}{aliasName}", Level::Document);
    let set_len = Custom::new(
        r"\setlength{\speakswidth}{\widthof{\speaksfont Long Name}}",
        Level::Document,
    );
    let add_to_length1 = Custom::new(r"\addtolength{\speakswidth}{\Dlabelsep}", Level::Document);
    let add_to_length2 = Custom::new(
        r"\addtolength{\speaksindent}{\speakswidth}",
        Level::Document,
    );

    let act = Custom::new(r"\act[] %[act name]", Level::Document);
    let scene = Custom::new(r"\scene[]%[scene name]", Level::Document);
    let end_comment = Custom::new(
        r" % To make character speak, use \<alias>speaks{text}",
        Level::Document,
    );
    let extra = Custom::new(
        r#"\usepackage[utf8]{inputenc}
\pagestyle{fancy}
\fancyhf{}
\lhead{\thetitle}
\rhead{\theauthor}
\rfoot{Page \thepage}"#,
        Level::Packages,
    );
    Elements![
        toc,
        newpage,
        character,
        set_len,
        add_to_length1,
        add_to_length2,
        act,
        scene,
        end_comment,
        extra,
        Package::new("dramatist"),
        Package::new("calc"),
        Package::new("fancyhdr")
    ]
}

// where you may declare the version of the template
fn version() -> Version {
    let mut version = Version::new();
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
pub fn generate_template() -> Template {
    // sets the metadata defaults of the template
    let mut metadata = Metadata::default();
    metadata.maketitle = true;
    // we will create a new template setting the name, description and metadata
    let mut template = Template::new(NAME, DESC, &metadata);
    // sets the template's version
    template.version = version();
    // push the elements into the template
    template.push_element_array(elements());
    template
}
