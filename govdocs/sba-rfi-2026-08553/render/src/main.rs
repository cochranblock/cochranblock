use pulldown_cmark::{html, Options, Parser};
use std::env;
use std::fs;
use std::process::ExitCode;

const COMMON_CSS: &str = r#"
@page { size: Letter; margin: 0; }
html { font-size: 10pt; }
html, body { width: 100%; }
body {
  line-height: 1.18;
  max-width: none;
  margin: 0;
  padding: 0.7in 0.7in 0.85in 0.7in;
  text-align: justify;
  hyphens: auto;
  box-sizing: border-box;
  -webkit-print-color-adjust: exact;
  print-color-adjust: exact;
}
h1, h2, h3, th, td { text-align: left; hyphens: manual; }
h1 { font-size: 1.35rem; margin: 0 0 0.3rem 0; line-height: 1.15; letter-spacing: -0.005em; }
h2 { font-size: 1.1rem;  margin: 0.55rem 0 0.18rem 0; line-height: 1.15; padding-bottom: 0.05rem; }
h3 { font-size: 0.98rem; margin: 0.4rem  0 0.1rem  0; line-height: 1.15; font-style: italic; }
p { margin: 0 0 0.3rem 0; }
ul, ol { margin: 0.1rem 0 0.3rem 1.15rem; padding: 0; }
li { margin-bottom: 0.08rem; }
li > p { margin: 0 0 0.08rem 0; }
code { font-size: 0.92em; padding: 0 0.18em; border-radius: 2px; }
pre  { padding: 0.3rem 0.5rem; border-radius: 3px; overflow-x: hidden; font-size: 0.88em; }
pre code { background: none; padding: 0; }
table { border-collapse: collapse; width: 100%; margin: 0.25rem 0 0.4rem 0; font-size: 0.95em; }
th, td { padding: 0.14rem 0.35rem; vertical-align: top; }
hr { border: 0; margin: 0.4rem 0; }
strong { font-weight: 700; }
h2, h3 { page-break-after: avoid; }
li, p  { orphans: 2; widows: 2; }
"#;

const LIGHT_CSS: &str = r#"
html, body { background: #fff; }
body { font-family: "DejaVu Serif", Georgia, "Times New Roman", serif; color: #111; }
h2 { border-bottom: 1px solid #888; }
code { font-family: "DejaVu Sans Mono", "Courier New", monospace; background: #f3f3f3; }
pre  { background: #f3f3f3; }
th, td { border: 1px solid #999; }
th { background: #eee; }
hr { border-top: 1px solid #888; }
a { color: #0a4a84; text-decoration: none; }
"#;

const DARK_CSS: &str = r#"
html, body { background: #0a0a0e; }
body { font-family: "DejaVu Sans", "Inter", "Helvetica Neue", system-ui, sans-serif; color: #e8e8e8; }
h1 { color: #00d9ff; }
h2 { color: #00d9ff; border-bottom: 1px solid rgba(255,255,255,0.18); }
h3 { color: #ffd866; }
strong { color: #f0f0f0; }
code { font-family: "DejaVu Sans Mono", "JetBrains Mono", "SF Mono", monospace; background: #11111a; color: #00d9ff; border: 1px solid rgba(255,255,255,0.06); }
pre  { background: #11111a; border: 1px solid rgba(255,255,255,0.08); }
pre code { color: #e8e8e8; border: 0; }
th, td { border: 1px solid rgba(255,255,255,0.12); }
th { background: #15152a; color: #ffd866; }
tr:nth-child(even) td { background: rgba(255,255,255,0.015); }
hr { border-top: 1px solid rgba(255,255,255,0.18); }
a { color: #00d9ff; text-decoration: none; }
/* watermark-style header banner via the H1 */
h1 {
  border-left: 3px solid #00d9ff;
  padding: 0.15rem 0 0.15rem 0.5rem;
  background: linear-gradient(90deg, rgba(0,217,255,0.06), rgba(0,217,255,0));
}
/* Cochran Block footer mark on every page via @page is not supported in chromium headless reliably;
   instead we inject a small page-corner mark via a body::after fixed element. */
body::after {
  content: "COCHRAN BLOCK / public domain / Unlicense / cochranblock.org";
  position: fixed;
  bottom: 0.3in;
  right: 0.5in;
  font-size: 7pt;
  color: rgba(0,217,255,0.55);
  letter-spacing: 0.08em;
  text-transform: uppercase;
  font-family: "DejaVu Sans Mono", monospace;
}
"#;

fn main() -> ExitCode {
    let mut dark = false;
    let mut positional: Vec<String> = Vec::new();
    for a in env::args().skip(1) {
        match a.as_str() {
            "--dark" => dark = true,
            "--light" => dark = false,
            _ => positional.push(a),
        }
    }
    if positional.len() < 2 {
        eprintln!("usage: rfi-md2html [--dark|--light] <in.md> <out.html>");
        return ExitCode::from(2);
    }
    let md = match fs::read_to_string(&positional[0]) {
        Ok(s) => s,
        Err(e) => { eprintln!("read {}: {}", &positional[0], e); return ExitCode::from(1); }
    };
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_FOOTNOTES);
    let parser = Parser::new_ext(&md, opts);
    let mut body = String::new();
    html::push_html(&mut body, parser);
    let theme_css = if dark { DARK_CSS } else { LIGHT_CSS };
    let body_class = if dark { "dark" } else { "light" };
    let out = format!(
        "<!doctype html><html><head><meta charset=\"utf-8\"><title>{}</title><style>{}{}</style></head><body class=\"{}\">{}</body></html>",
        html_escape(&positional[0]),
        COMMON_CSS,
        theme_css,
        body_class,
        body
    );
    if let Err(e) = fs::write(&positional[1], out) {
        eprintln!("write {}: {}", &positional[1], e);
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}
