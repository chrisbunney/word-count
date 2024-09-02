use mywc;
use std::env;

pub fn main() {
    let inputs = env::args();
    let mut total_nlines = 0;
    let mut total_nwords = 0;
    let mut total_nchars = 0;

    // first agument in args list is always the program name:
    for infile in inputs.skip(1) {
        let (nwords, nchars, nlines) = mywc::from_file(&infile);
        println!("{} {} {}  {}", nlines, nwords, nchars, infile);
        total_nwords += nwords;
        total_nlines += nlines;
        total_nchars += nchars;
    }

    if env::args().len() > 2 {
        println!("{} {} {}  total", total_nlines, total_nwords, total_nchars);
    }
}
