// This is where your template will be created.
use texcore::template::*;

use texcore::*;

// Name of the template
static NAME: &str = "news";
// Description of the template
static DESC: &str = r#"Write your own newspaper article"#;

const MAIN: &str = r#"%----------------------------------------------------------------------------------------
%	HEADER IMAGE
%----------------------------------------------------------------------------------------

\begin{figure}[H]
% Add logo here
% \centering\includegraphics[width=0.3\linewidth]{logo.png}
\end{figure}

%----------------------------------------------------------------------------------------
%	SIDEBAR - FIRST PAGE
%----------------------------------------------------------------------------------------

\begin{minipage}[t]{.30\linewidth} % Mini page taking up 30% of the actual page
\begin{mdframed}[style=sidebar,frametitle={}] % Sidebar box

%-----------------------------------------------------------

\hypertarget{contents}{\textbf{{\large In This Issue\ldots}}} % \hypertarget provides a label to reference using \hyperlink{label}{link text}
\begin{itemize}
\item \hyperlink{firstnews}{The first news headline!} % These link to their appropriate sections in the newsletter
\item \hyperlink{secondnews}{The second news headline}
\item \hyperlink{thirdnews}{The third news headline\ldots}
\item \hyperlink{descriptivebox}{A descriptive box!}
\item \hyperlink{quotation}{Customer quotation}
\end{itemize}

\centerline {\rule{.75\linewidth}{.25pt}} % Horizontal line

%-----------------------------------------------------------

\textbf{Quisque ut nisl ut enim!}

Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae nulla.  Quisque ornare mattis odio eget bibendum. Integer ac nisl in est consectetur:

\begin{enumerate}
\item Mauris hendrerit nibh vitae \textsl{Mauris Porttitor Malesuada} faucibus.
\item Nam dignissim \textsl{Sollicitudin Nunc} id.
\item Scelerisque urna imperdiet non \textsl{varius} ipsum primis \textsl{posuere}.
\item Bibendum \textsl{orci} consectetur.
\end{enumerate}

%-----------------------------------------------------------

\textbf{Quis Dapibus Ipsum!}

Phasellus malesuada nisl at sapien vehicula at porttitor mi egestas ornare quis lobortis eros volutpat nunc viverra viverra ligula, sit amet posuere lectus aliquam quis. Vel at dolor \href{http://www.example.com}{elementum sapien}, mollis sit amet.

\textbf{Ornare Justo Ultricies}

Aliquam tristique sapien ut neque condimentum fermentum. In ultricies dictum massa, sit amet volutpat felis rutrum sit amet. Nulla vulputate leo et purus \href{http://www.example.com}{Ornare Justo Ultricies}. Consectetur iaculis eros \href{http://www.example.com}{Ornare Justo} non et.

%-----------------------------------------------------------

\captionof*{table}{Table Caption}
\begin{tabular}{llr}
\toprule
\multicolumn{2}{c}{Name} \\
\cmidrule(r){1-2}
First & Last & Grade \\
\midrule
John & Doe & $7.5$ \\
Richard & Miles & $2$ \\
\bottomrule
\end{tabular}

%-----------------------------------------------------------

\end{mdframed}
\end{minipage}\hfill % End the sidebar mini page
%
%----------------------------------------------------------------------------------------
%	MAIN BODY - FIRST PAGE
%----------------------------------------------------------------------------------------
%
\begin{minipage}[t]{.66\linewidth} % Mini page taking up 66% of the actual page

\hypertarget{firstnews}{\heading{The First News Headline}{6pt}} % \hypertarget provides a label to reference using \hyperlink{label}{link text}

In vel accumsan nisi. Sed eget justo a erat volutpat varius ut non quam. Nulla placerat eros id nibh laoreet pulvinar. Quisque semper.

\begin{center}
\parbox[t]{.70\linewidth}{\textsl{Pellentesque gravida volutpat convallis. Ut sollicitudin egestas dictum. Pellentesque in nisi faucibus ullamcorper eu ac magna!}}
\end{center}

Duis eu \textsl{Felis eget}, purus bibendum elementum. Donec sed nunc sit amet ante congue aliquet tempus vitae ligula. Vestibulum dapibus malesuada purus, nec tincidunt lacus mollis ut. Nunc luctus accumsan tortor quis molestie. Donec erat nisl, auctor nec semper imperdiet, adipiscing ut ante. Vestibulum id metus libero, a vestibulum diam.

\begin{wrapfigure}[7]{l}[0pt]{0pt} % In-line figure with text wrapping around it
% \includegraphics[width=0.3\textwidth]{placeholder.jpg}
\end{wrapfigure}

\textsl{Vivamus condimentum interdum} metus at varius vehicula, elit nunc sollicitudin enim, at scelerisque lorem urna vitae metus. Donec hendrerit consectetur tincidunt. Duis a magna justo. Nunc non mauris mi, eget commodo leo. Maecenas gravida lacus nunc, at viverra quam. Duis a turpis at lectus ultricies condimentum ut ut enim. Duis a magna justo. Nunc non mauris mi, eget commodo leo. Maecenas gravida lacus nunc, at viverra quam. Duis a turpis at lectus ultricies condimentum ut ut enim.

Sed diam nisi, tristique ut dapibus a, volutpat vitae nisi. Vivamus aliquam quam sed mi rhoncus commodo. Suspendisse massa mauris, ullamcorper ut hendrerit vitae, ultrices eget ligula.

Donec hendrerit consectetur tincidunt, nec gravida \href{http://www.example.com}{\textit{Duis a Magna Justo}} auctor nec, semper \href{http://www.example.com/}{Nec Gravida Rutrum} nulla nulla \href{http://www.example.com}{Elementum}, neque eu vulputate.

%-----------------------------------------------------------

\hypertarget{secondnews}{\heading{The Second News Headline}{6pt}} % \hypertarget provides a label to reference using \hyperlink{label}{link text}

Aenean turpis libero, accumsan vitae sagittis eu, ornare eu lacus. Sed id odio eget velit congue porttitor. Cras quis neque non magna fringilla dapibus:

\begin{itemize}
\item morbi vulputate dapibus sapien
\item cras vel nulla libero, in varius augue
\item pellentesque quis nisl id
\end{itemize}

Donec nulla elit, vulputate vel pharetra et, tempor ut nisl vivamus. Sapien lectus porttitor urna, sit amet lobortis odio eros at metus.

Integer orci lorem, aliquam in dapibus at, tincidunt vitae felis nullam nec sem tellus, nec condimentum diam.

Donec massa dolor, fermentum eget sagittis ut, sagittis placerat lacus. Donec id purus nunc fusce dignissim gravida condimentum.

Lorem ipsum dolor sit amet, consectetur adipiscing elit imperdiet vel pellentesque sed.

Ut condimentum ornare rhoncus lorem ipsum dolor sit amet consectetu. Adipiscing elit \href{http://www.example.com}{Nunc Ultrices}.

\end{minipage} % End the main body - first page mini page

%----------------------------------------------------------------------------------------
%	MAIN BODY - SECOND PAGE
%----------------------------------------------------------------------------------------

\begin{minipage}[t]{.66\linewidth} % Mini page taking up 66% of the actual page

\hypertarget{thirdnews}{\heading{The Third News Headline}{6pt}} % \hypertarget provides a label to reference using \hyperlink{label}{link text}

\begin{multicols}{2} % Two-column layout
Morbi tempus bibendum felis a consectetur. In commodo mollis odio et pharetra. In et risus at augue ultricies cursus. Sed sit amet pretium mauris. Vivamus eu luctus ante.

Nunc facilisis, velit nec elementum ultrices, elit justo egestas libero, feugiat vestibulum nulla velit vel eros. Ut feugiat dictum aliquet. In hac habitasse platea dictumst. Pellentesque ullamcorper consequat felis. Aenean sit \textsl{amet} orci non felis dignissim ultricies at quis eros. Sed mi lectus, suscipit nec tempor ac, bibendum non metus.

Morbi et sem vel lorem luctus viverra. Mauris elit metus, lobortis eget fermentum vitae, auctor ut felis. Sed magna libero, sagittis sed congue et, dignissim id lorem.

Vestibulum congue, magna vel sodales rutrum, nisl elit facilisis dolor, vitae sodales purus augue eget justo. Morbi vehicula pharetra felis a elementum. Quisque ipsum metus, rhoncus ac consequat in, bibendum at neque. Aliquam ac nunc turpis. In in faucibus velit. Phasellus tempor, tortor ac malesuada mattis, arcu velit \textsl{venenatis} neque, eget vulputate lorem tellus sed lacus. Nam id dictum lorem.

Etiam scelerisque nulla at felis ultricies eget rhoncus justo scelerisque. Integer tincidunt accumsan neque, mattis varius nisi molestie sit amet. Praesent vel est eget velit pulvinar rutrum ac ut est. Curabitur purus augue \textsl{Luctrra Vitae / Felis\dots}  a facilisis. Proin semper dignissim justo, at laoreet massa scelerisque vel. Donec erat erat, laoreet eu euismod eu, rutrum id est. Nunc eget odio nisi.

Suspendisse in quam at lectus rutrum laoreet sed eu lacus. Nam viverra commodo pellentesque suspendisse interdum adipiscing era id tempor nisi laoreet nec.

Ut sollicitudin urna eu urna  \texttt{192.168.*.*} ornare congue. Maecenas eget leo et metus convallis lacinia.

Quisque mi metus, fringilla eget volutpat id, facilisis ac arcu. Maecenas mauris mi, mattis at varius vitae, tincidunt eu quam. Aliquam bibendum quam porttitor arcu ultricies volutpat. In hac habitasse dictumst.

\BackToContents % Link back to the contents of the newsletter

\end{multicols}

%----------------------------------------------------------------------------------------
%	IN-TEXT BOX
%----------------------------------------------------------------------------------------

\begin{mdframed}[style=intextbox,frametitle={}] % Sidebar box

\hypertarget{descriptivebox}{\heading{Descriptive Box}{0pt}} % \hypertarget provides a label to reference using \hyperlink{label}{link text}

Nam in turpis scelerisque libero luctus tincidunt sed faucibus lorem ac nisi rhoncus hendrerit. Praesent semper dui non justo rhoncus consequat. Mauris egestas tempus ligula:
\begin{enumerate}
\item Quisque mi metu.
\item Sem iaculis molestie vulputate.
\item Suscipit accumsan dui metus sed.
\item Molestie sed ullamcorper quis, venenatis nec lorem.
\item Nec pulvinar \textsl{Rutrum Cursus / Ante\dots} command.
\item Donec sit amet sem ligula,  \textsl{non} posuere neque.
\item In hac habitasse nibh ultricies pellentesque elit risus sollicitudin velit.
\item Aliquam nec metus sed.
\end{enumerate}

\BackToContents % Link back to the contents of the newsletter

\end{mdframed}

%----------------------------------------------------------------------------------------
%	QUOTATION
%----------------------------------------------------------------------------------------

\hypertarget{quotation}{{\large Customer Quotes}} % \hypertarget provides a label to reference using \hyperlink{label}{link text}

\begin{quote}
\textsl{``Praesent semper dui non justo rhoncus consequat. Mauris egestas tempus ligula, nec consequat arcu imperdiet eu. Mauris molestie lacinia enim, in dictum enim sagittis ac!!!!''} --- \textrm{John Smith}
\end{quote}

%----------------------------------------------------------------------------------------

\end{minipage}\hfill % End of the main body - second page mini page
\begin{minipage}[t]{.30\linewidth} % Mini page taking up 30% of the actual page

%----------------------------------------------------------------------------------------
%	SIDEBAR - SECOND PAGE
%----------------------------------------------------------------------------------------

\begin{mdframed}[style=sidebar,frametitle={}] % Sidebar box

\heading{Specifications}{0pt}

Magna ipsum:

\begin{enumerate}
\item Maecenas fringilla mi quis dolor vestibulum bibendum scelerisque erat rhoncus.
\item Ut id est metus, non pharetra leo Etiam in lorem sit amet sapien semper iaculis vel eget risus. Morbi consectetur gravida eros, non tincidunt ligula hendrerit vel.
\item Donec venenatis metus vitae urna malesuada vestibulum vitae eget mauris mauris sed orci dui, eget fringilla arcu. Nunc scelerisque tortor ut enim laoreet eget scelerisque ante ultricies.
\item Ut nec lacus vel ante molestie ac nec eros:

\textsl{Ipsum auctor} sit amet sapien semper.

\textsl{Eget fringilla} sed ut augue nec turpis vestibulum imperdiet.

\textsl{Tincidunt} ut id est metus, non pharetra leo.

\textsl{Vel velit venenatis } morbi consequat elementum eros, at pretium lectus gravida quis.

\textsl{Luctus nisi} morbi consectetur gravida eros, non tincidunt ligula hendrerit vel.

\textsl{Eget Odio} zunc scelerisque tortor ut enim laoreet eget scelerisque ante ultricie mollis lorem a sapien scelerisque varius.
\end{enumerate}

\BackToContents % Link back to the contents of the newsletter

\end{mdframed}\hfill

%----------------------------------------------------------------------------------------

\centering
\begin{minipage}[t]{.95\linewidth}
\textbf{Contact Information:}\\
Vivamus Technologies, Ltd.\\
1234 Street Name,\\
City, Country, Post Code\\
\href{http://www.example.com}{http://www.ornarejustoultricies.com}\\
\href{http://www.example.com}{http://www.elementumsapien.com}\\
\href{http://www.example.com}{http://www.nuncultrices.com}
\end{minipage}

%----------------------------------------------------------------------------------------

\end{minipage} % End of the sidebar mini page

%----------------------------------------------------------------------------------------

    "#;

const PKG: &str = r#"%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% Professional Newsletter Template
% Structural Definitions File
% Version 1.0 (09/03/14)
%
% Created by:
% Vel (vel@latextemplates.com)
%
% This file has been downloaded from:
% http://www.LaTeXTemplates.com
%
% License:
% CC BY-NC-SA 3.0 (http://creativecommons.org/licenses/by-nc-sa/3.0/)
%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

%----------------------------------------------------------------------------------------
%	REQUIRED PACKAGES
%----------------------------------------------------------------------------------------

\usepackage{graphicx} % Required for including images
\usepackage{microtype} % Improved typography
\usepackage{multicol} % Used for the two-column layout of the document
\usepackage{booktabs} % Required for nice horizontal rules in tables
\usepackage{wrapfig} % Required for in-line images
\usepackage{float} % Required for forcing figures not to float with the [H] parameter

%------------------------------------------------
% Fonts

\usepackage{charter} % Use the Charter font as the main document font
\usepackage{courier} % Use the Courier font for \texttt (monospaced) only
\usepackage[T1]{fontenc} % Use T1 font encoding

%------------------------------------------------
% List Separation

\usepackage{enumitem} % Required to customize the list environments
\setlist{noitemsep,nolistsep} % Remove spacing before, after and within lists for a compact look

%------------------------------------------------
% Figure and Table Caption Styles

\usepackage{caption} % Required for changing caption styles
\captionsetup[table]{labelfont={bf,sf},labelsep=period,justification=justified} % Specify the table caption style
\captionsetup[figure]{labelfont={sf,bf},labelsep=period,justification=justified, font=small} % Specify the figure caption style
\setlength{\abovecaptionskip}{10pt} % Whitespace above captions

%------------------------------------------------
% Spacing Between Paragraphs

\makeatletter
\usepackage{parskip}
\setlength{\parskip}{6pt}
\newcommand{\@minipagerestore}{\setlength{\parskip}{6pt}}
\makeatother

%----------------------------------------------------------------------------------------
%	PAGE MARGINS AND SPACINGS
%----------------------------------------------------------------------------------------

\textwidth = 7 in % Text width
\textheight = 10 in % Text height
\oddsidemargin = -18pt % Left side margin on odd pages
\evensidemargin = -18pt % Left side margin on even pages
\topmargin = -36pt % Top margin
\headheight = 0pt % Remove the header by setting its space to 0
\headsep = 0pt % Remove the space between the header and top of the page
\parskip = 4pt % Space between paragraph
\parindent = 0.0in % Paragraph indentation
\pagestyle{empty} % Disable page numbering

%----------------------------------------------------------------------------------------
%	COLORS
%----------------------------------------------------------------------------------------

\usepackage[dvipsnames,svgnames]{xcolor} % Required to specify custom colors

\definecolor{altncolor}{rgb}{.8,0,0} % Dark red
%\definecolor{altncolor}{rgb}{.2,.4,.8} % Dark blue
%\definecolor{altncolor}{rgb}{.84,.16,.16} % Red

\usepackage[colorlinks=true, linkcolor=altncolor, anchorcolor=altncolor, citecolor=altncolor, filecolor=altncolor, menucolor=altncolor, urlcolor=altncolor]{hyperref} % Use the color defined above for all links

%----------------------------------------------------------------------------------------
%	BOX STYLES
%----------------------------------------------------------------------------------------

\usepackage[framemethod=TikZ]{mdframed}% Required for creating boxes
\mdfdefinestyle{sidebar}{
    linecolor=black, % Outer line color
    outerlinewidth=0.5pt, % Outer line width
    roundcorner=0pt, % Amount of corner rounding
    innertopmargin=10pt, % Top margin
    innerbottommargin=10pt, % Bottom margin
    innerrightmargin=10pt, % Right margin
    innerleftmargin=10pt, % Left margin
    backgroundcolor=white, % Box background color
    frametitlebackgroundcolor=white, % Title background color
    frametitlerule=false, % Title rule - true or false
    frametitlerulecolor=white, % Title rule color
    frametitlerulewidth=0.5pt, % Title rule width
    frametitlefont=\Large, % Title heading font specification
    font=\small
}

\mdfdefinestyle{intextbox}{
    linecolor=black, % Outer line color
    outerlinewidth=0.5pt, % Outer line width
    roundcorner=10pt, % Amount of corner rounding
    innertopmargin=7pt, % Top margin
    innerbottommargin=7pt, % Bottom margin
    innerrightmargin=7pt, % Right margin
    innerleftmargin=7pt, % Left margin
    backgroundcolor=white, % Box background color
    frametitlebackgroundcolor=white, % Title background color
    frametitlerule=false, % Title rule - true or false
    frametitlerulecolor=white, % Title rule color
    frametitlerulewidth=0.5pt, % Title rule width
    frametitlefont=\Large % Title heading font specification
}

%----------------------------------------------------------------------------------------
%	HEADING STYLE
%----------------------------------------------------------------------------------------

\newcommand{\heading}[2]{ % Define the \heading command
\vspace{#2} % White space above the heading
{\begin{center}\Large\textbf{#1}\end{center}} % The heading style
\vspace{#2} % White space below the heading
}

\newcommand{\BackToContents}{\hyperlink{contents}{{\small Back to Contents}}} % Define a command for linking back to the contents of the newsletter
    "#;

// Used to distribute the name
pub fn name() -> String {
    NAME.to_string()
}

// where all the template elements will be created
fn elements() -> Vec<Element<Any>> {
    Elements![
        Custom::new(MAIN, Level::Document),
        Custom::new(PKG, Level::Packages)
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
