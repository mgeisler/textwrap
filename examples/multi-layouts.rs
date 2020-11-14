use textwrap::{NoHyphenation, Options};

#[cfg(feature = "hyphenation")]
use hyphenation::{Language, Load, Standard};

// Pretend this is an external crate
mod library {
    use textwrap::{Options, WordSplitter};

    #[derive(Debug, Default)]
    pub struct Layout<'a> {
        // Use trait-objects which can be easily converted from any concrete `Options`
        styles: Vec<Box<Options<'a, dyn WordSplitter + 'a>>>,
    }

    impl<'a> Layout<'a> {
        pub fn new() -> Self {
            Default::default()
        }

        // Similar signature like `wrap` has, so it takes (nearly) everything that `warp` takes.
        pub fn add<S: WordSplitter + 'a, T: Into<Options<'a, S>>>(&mut self, option: T) {
            self.styles.push(Box::new(option.into()));
        }

        pub fn print(&self, text: &str) {
            // Now we can easily go over all the arbitrary Options and use them for layouting.
            for opt in self.styles.iter() {
                println!();

                // the debug output of the hyphenation is very long
                //println!("Options: {:#?}", opt);

                println!("{:0width$}", 0, width = opt.width);

                // Just use the textwrap functions as usual.
                // However, we have to first coerce it into a trait-object
                let dyn_opt: &Options<'a, dyn WordSplitter> = opt;
                println!("{}", textwrap::fill(text, dyn_opt));
            }
        }
    }
}

pub fn main() {
    // pretend we are a user of the library module above

    // Just some options (see below for usage)
    let some_opt = Options::new(25).initial_indent("----");

    // The struct from the 'library' that we are using
    let mut layout = library::Layout::new();

    // Add some arbitrary options. We can use here the same as for `fill` & `wrap`.

    // Plain Options
    layout.add(Options::new(20));

    // usize
    layout.add(30);

    // Customized Options
    let opt = Options::new(30);
    let opt = opt.subsequent_indent("****");
    layout.add(opt.clone()); // notice, here we pass opt by-value instead of by-reference

    // We can use boxed splitters too (however, we have to coerce the Options)
    let opt: Options = opt.splitter(Box::new(NoHyphenation));
    layout.add(opt);

    // We can also pass-in references, however, those need to outlive the local
    // `layout`, so here, it must be declared before `layout` (drop order).
    layout.add(&some_opt);

    // If you like, you can download more dictionaries from
    // https://github.com/tapeinosyne/hyphenation/tree/master/dictionaries
    // Place the dictionaries in the examples/ directory. Here we
    // just load the embedded en-us dictionary.
    #[cfg(feature = "hyphenation")]
    for lang in &[Language::EnglishUS] {
        let dictionary = Standard::from_embedded(*lang).or_else(|_| {
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("examples")
                .join(format!("{}.standard.bincode", lang.code()));
            Standard::from_path(*lang, &path)
        });

        if let Ok(dict) = dictionary {
            layout.add(Options::with_splitter(25, dict));
        }
    }

    let example = "Memory safety without garbage collection. \
                   Concurrency without data races. \
                   Zero-cost abstractions.";

    // Printout above text in all different layouts
    layout.print(example);
}
