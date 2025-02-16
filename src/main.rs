use std::fs;
use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: tex \"LaTeX code\"");
        return;
    }
    
    let latex = &args[1];
    render_latex(latex);
}

fn render_latex(latex: &str) {
    let tex_content = format!(
        "\\documentclass[preview]{{standalone}}
        \\usepackage{{amsmath}}
        \\usepackage{{xcolor}}  % For text color
        \\begin{{document}}
        \\color{{white}}  % Set text color to white
        \\begin{{equation*}}
        {}
        \\end{{equation*}}
        \\end{{document}}",
        latex
    );
    
    fs::write("temp.tex", tex_content).expect("Failed to write LaTeX file");
    
    Command::new("tectonic")
        .arg("temp.tex")
        .output()
        .expect("Failed to compile LaTeX");
    
    Command::new("convert")
        .args(["-density", "300", "temp.pdf", "-quality", "100", "-trim", "-colorspace", "RGB", "temp.png"])
        .output()
        .expect("Failed to convert PDF to image");

    Command::new("viu")
        .args(["-h", "5", "temp.png"])  
        .status()
        .expect("Failed to display image in terminal");
}
