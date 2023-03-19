// This is where your template will be created.
use texcore::template::*;
use texcore::TextType::Normal;
use texcore::*;

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

const PKGEXTRA: &str = r#"\definecolor{codegreen}{rgb}{0,0.6,0}
\definecolor{codegray}{rgb}{0.5,0.5,0.5}
\definecolor{codepurple}{rgb}{0.58,0,0.82}
\definecolor{backcolour}{rgb}{0.95,0.95,0.92}
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
\lstset{language=c}
\lstset{style=lang_style}
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
    let pkgextra = Custom::new(PKGEXTRA, Level::Packages);
    // This macro converts all TexCore types that implement
    // the `Tex` trait to Element<Any>
    Elements![
        intro,
        example,
        comment,
        pkgextra,
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
    template
}
