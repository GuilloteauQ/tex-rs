extern crate RusTex;
use RusTex::core::*;
use RusTex::latex_file::*;
use RusTex::writable::Writable;

fn main() {
    let mut f = new_latex_file("out.tex");
    f.change_title("Example of use of RusTex");
    f.change_author("GuilloteauQ");
    f.begin_document();

    let mut abstract_bloc = Core::new_bloc("abstract");
    abstract_bloc.add(Core::new_raw_text("This document is an example of use of RusTex"));
    abstract_bloc.write_latex(&mut f);

    let mut section = Core::new_section("Introduction");
    section.add(Core::new_raw_text("RusTex allows you to create sections"));
    section.add(Core::new_subsection("... and subsection, and more!"));
    section.write_latex(&mut f);

    let mut sec = Core::new_section("Examples");
    let mut enume = Core::new_bloc("itemize");

    let countries = vec!["France", "UK", "Germany", "Ialy"];
    sec.add(Core::new_raw_text("Here is some countries in Europe"));
    for country in countries.iter() {
        enume.add(Core::item(Core::new_raw_text(*country)));
    }

    sec.add(enume);
    sec.write_latex(&mut f);

    f.write_footer();
}
