extern crate tex_rs;
use std::io;
use tex_rs::core::*;
use tex_rs::latex_file::*;
use tex_rs::writable::Writable;

fn main() {
    let mut variables = vec![
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    ];
    let queries = vec![
        "nationality",
        "your name",
        "first interest",
        "second interest",
        "valuable course",
    ];
    let stdin = io::stdin();

    for (mut var, query) in variables.iter_mut().zip(queries.iter()) {
        println!(">>> Enter {}:", query);
        let _ = stdin.read_line(&mut var);
    }
    let nationality = &variables[0];
    let name = &variables[1];
    let interest1 = &variables[2];
    let interest2 = &variables[3];
    let course = &variables[4];

    let mut f = new_latex_file("cover_letter.tex");
    f.title("Cover Letter");
    f.author(&name);
    f.begin_document();

    let mut presentation = Core::paragraph("");
    presentation.add(Core::text(format!("I am a {} student in Computer Science, interested in {} and {}. My on-going degree has already given me a full set of skills to adress any difficulty that I could encounter during this internship.", nationality, interest1, interest2)));
    presentation.add(Core::text(format!("In particular, the course on {} provided to me a great understanding of this internship's topic.", course)));

    let mut bs = Core::paragraph("");

    bs.add(Core::text("My degree and my previous internships made me able to work and communicate in a team, as well as being an independent worker. I am eager to learn from experienced people, and I like to discover and pick up new skills. I am not afraid of responsabilities, and to take initiatives."));

    let mut thanks = Core::paragraph("");
    thanks.add(Core::text("Thank you for taking the time to consider my application. I will be at your disposal for any question that you may have. I look forward to hearing from you."));

    let mut end = Core::paragraph("");
    end.add(Core::text("Respectfully,"));

    let mut signature = Core::paragraph("");
    signature.add(Core::text(name));

    presentation.write_latex(&mut f);
    bs.write_latex(&mut f);
    thanks.write_latex(&mut f);
    end.write_latex(&mut f);
    signature.write_latex(&mut f);

    f.write_footer();
}
