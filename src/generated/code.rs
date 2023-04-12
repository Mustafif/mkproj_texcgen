// This is where your template will be created.
use texcore::template::*;
use texcore::TextType::Normal;
use texcore::*;
use texcore_traits::Options::Curly;
use texcore_traits::{ExtraOptions, Options};

// Name of the template
static NAME: &str = "code";
// Description of the template
static DESC: &str = r#"Code Documentation Focused"#;

const CODE: &str = r#"#include <stdio.h>
int main()
{
    printf("Hello World!");
    return 0;
}
/*Comments*/
"#;

fn packages() -> Vec<Element<Any>> {
    // we will be defining multiple colours
    fn define_color(c: &str) -> Custom {
        let s = format!(r"\definecolor{{{c}}}");
        Custom::new(&s, Level::Packages)
    }
    // used to define the extra color options
    fn rgb_option(r: f64, g: f64, b: f64) -> Vec<Options> {
        let rgb_option = Curly("rbg".to_string());
        let color = Curly(format!("{r},{g}, {b}"));
        vec![rgb_option, color]
    }

    let mut green = define_color("codegreen");
    green.modify_element(rgb_option(0.0, 0.6, 0.0));

    let mut gray = define_color("codegray");
    gray.modify_element(rgb_option(0.5, 0.5, 0.5));

    let mut purple = define_color("codepurple");
    purple.modify_element(rgb_option(0.58, 0.0, 0.82));

    let mut backcolour = define_color("backcolour");
    backcolour.modify_element(rgb_option(0.95, 0.95, 0.92));

    let style = Custom::new(STYLE, Level::Packages);

    let lang_set = Custom::new(r"\lst{language=c}", Level::Packages);
    let style_set = Custom::new(r"\lstset{style=lang_stle}", Level::Packages);

    Elements![green, gray, purple, backcolour, style, lang_set, style_set]
}

const STYLE: &str = r#"
\lstdefinestyle{lang_style}{
    backgroundcolor=\color{backcolour},
    commentstyle=\color{codegreen},
    keywordstyle=\color{magenta},
    numberstyle=\tiny\color{codegray},
    stringstyle=\color{codepurple},
    basicstyle=\ttfamily\footnotesize,
    breakatwhitespace=false,
    breaklines=true,
    captionpos=b,
    keepspaces=true,
    numbers=left,
    numbersep=5pt,
    showspaces=false,
    showstringspaces=false,
    showtabs=false,
    tabsize=2
}
"#;

// Used to distribute the name
pub fn name() -> String {
    NAME.to_string()
}

// where all the template elements will be created
fn elements() -> Vec<Element<Any>> {
    let intro = Header::new("Intro", 1);
    let mut example = Environment::new("lstlisting");
    example.push(Element::from(Text::new(CODE, Normal)));
    let comment = Custom::new(
        r#"% To use external code, use the following command: \lstinputlisting{file.c}"#,
        Level::Document,
    );
    // This macro converts all TexCore types that implement
    // the `Tex` trait to Element<Any>
    Elements![
        intro,
        example,
        comment,
        Package::new("graphicx"),
        Package::new("listings"),
        Package::new("xcolor"),
        Package::new("amsmath")
    ]
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
    // push the extra packages into template
    template.push_element_array(packages()).await;
    template
}
