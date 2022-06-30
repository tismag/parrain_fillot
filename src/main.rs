use std::{env, fs::*, io::Write, path::Path, process};

/* CONDITIONS DE FONCTIONNEMENT
    - Plus les questiosn ont d'options de réponse, moins l'algorithme est efficace
    => Prioriser le nombre de questions au nombre d'options de réponses

    - Plus de fillots que de parrains

    - Même questionnaire pour les parrains et les fillots

    - Le CSV doit contenir une 1ère ligne et une 1ère colonne inutiles
    => En export csv depuis google form, la 1ère ligne réitère les questions et la première colonne contient l'horodateur, on veut pas ces données

    - Pas de virgules ni de saut de ligne dans les options de réponse
*/

/*
Il risque d'y avoir des parrains laissés seuls à la fin du programme, il suffit alors de retoucher les résultats à la main.
De toute façon, il est prévu de retoucher les résultats si certains parrains ont des requêtes particulières
*/

//################################
// PARSING CSV
//################################

fn read_from_file(filename: String) -> String {
    let path = Path::new(&filename);
    let content = match read_to_string(path) {
        Ok(x) => x,
        Err(_) => panic!("Error occured while reading the file"),
    };
    content
}

fn parse_csv(content: &String) -> Vec<Vec<String>> {
    let mut iter = content.chars();

    let mut answer = String::new();
    let mut line: Vec<String> = Vec::new();
    let mut data: Vec<Vec<String>> = Vec::new();
    let mut eof: bool = false;

    let mut c: Option<char>;
    while !eof {
        c = iter.next();
        match c {
            Some(',') => {
                line.push(answer);
                answer = "".to_string();
            }
            Some('\n') => {
                line.push(answer);
                data.push(line);
                answer = "".to_string();
                line = Vec::new();
            }
            Some(ch) => {
                answer.push(ch);
            }
            None => {
                line.push(answer.clone());
                data.push(line.clone());
                eof = true;
            }
        }
    }
    return data;
}

//################################
// ASSOCIATION PARRAINS-FILLOTS
//################################

#[derive(Debug, Clone)]
struct Parrain {
    fillots: Vec<(String, usize)>,
    infos: String,
    choix: Vec<String>,
}

impl Parrain {
    fn new() -> Parrain {
        Parrain {
            fillots: Vec::new(),
            infos: String::new(),
            choix: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct Fillot {
    parrains_prefs: Vec<(String, usize)>,
    infos: String,
    choix: Vec<String>,
}

impl Fillot {
    fn new() -> Fillot {
        Fillot {
            parrains_prefs: Vec::new(),
            infos: String::new(),
            choix: Vec::new(),
        }
    }
}

fn parse_parrains(p: &Vec<Vec<String>>, decal: usize) -> Vec<Parrain> {
    let mut parrains: Vec<Parrain> = Vec::new();
    let mut par: Parrain;

    for line in p {
        par = Parrain::new();
        par.infos = line[1..decal+1].join(" ");
        par.choix = line[decal+1..].to_vec();
        parrains.push(par);
    }
    parrains
}

fn parse_fillots(f: &Vec<Vec<String>>, decal: usize) -> Vec<Fillot> {
    let mut fillots: Vec<Fillot> = Vec::new();
    let mut fi: Fillot;

    for line in f {
        fi = Fillot::new();
        fi.infos = line[1..decal+1].join(" ");
        fi.choix = line[decal+1..].to_vec();
        fillots.push(fi);
    }
    fillots
}

fn calcul_parrains_pref(p: &Vec<Parrain>, f: &mut Vec<Fillot>) {
    let answer_len = f[0].choix.len();
    let mut p_pref_len: usize;
    let mut somme: usize = 0;

    for fillot in f {
        for parrain in p {
            for i in 0..answer_len {
                if fillot.choix[i] == parrain.choix[i] {
                    somme += 1;
                }
            }
            p_pref_len = fillot.parrains_prefs.len();
            if p_pref_len == 0 {
                fillot.parrains_prefs.push((parrain.infos.clone(), somme));
            } else {
                for i in 0..p_pref_len {
                    if fillot.parrains_prefs[i].1 < somme {
                        fillot
                            .parrains_prefs
                            .insert(i, (parrain.infos.clone(), somme));
                        break;
                    }
                }
            }
        }
    }
}

// Returns the index of the Parrain with same information in a list of Parrains
fn get_parrain(liste: &mut Vec<Parrain>, infos_parrain: String) -> usize {
    for i in 0..liste.len() {
        if liste[i].infos == infos_parrain {
            return i;
        }
    }
    println!("Could not find parrain");
    process::exit(2);
}

// Returns the Fillot with same information in a list of Fillots and removes it from the list
fn get_fillot(liste: &mut Vec<Fillot>, infos_fillot: &String) -> Fillot {
    for i in 0..liste.len() {
        if liste[i].infos == *infos_fillot {
            let ret = liste[i].clone();
            liste.remove(i);
            return ret;
        }
    }
    println!("Could not find fillot");
    process::exit(2);
}

/*
This function is based Gale-Shapley stable matching algorithm
It is adapted for the situation as every Parrain can have multiple Fillot
*/
fn stable_matching(p: &mut Vec<Parrain>, mut f: Vec<Fillot>, rapport_f_p: usize) {
    let married: &mut Vec<Fillot> = &mut Vec::new();
    while f.len() != 0 {
        let index_p_pref = get_parrain(p, f[0].parrains_prefs[0].0.clone());
        let p_pref = &mut p[index_p_pref];
        if p_pref.fillots.len() < rapport_f_p {
            p_pref
                .fillots
                .push((f[0].infos.clone(), f[0].parrains_prefs[0].1));
            f[0].parrains_prefs.remove(0);
            married.push(f[0].clone());
            f.remove(0);
        } else {
            for i in 0..p_pref.fillots.len() {
                if f[0].parrains_prefs[0].1 > p_pref.fillots[i].1 {
                    f.push(get_fillot(married, &p_pref.fillots[i].0));
                    p_pref.fillots.remove(i);
                    p_pref
                        .fillots
                        .push((f[0].infos.clone(), f[0].parrains_prefs[0].1));
                    f[0].parrains_prefs.remove(0);
                    married.push(f[0].clone());
                    f.remove(0);
                }
            }
        }
    }
}

fn write_results(results: Vec<Parrain>) {
    let path = "parrains_fillots.txt";

    let mut output = File::create(path).unwrap();
    let mut write_output;

    for pf in results {
        write_output = writeln!(output, "{} {{", pf.infos);
        match write_output {
            Err(_) => {
                println!("Error while writing the results");
                process::exit(1);
            }
            Ok(_) => {}
        }
        for f in pf.fillots {
            write_output = writeln!(output, "   {}", f.0);
            match write_output {
                Err(_) => {
                    println!("Error while writing the results");
                    process::exit(1);
                }
                Ok(_) => {}
            }
        }
        write_output = writeln!(output, "}}\n");
        match write_output {
            Err(_) => {
                println!("Error while writing the results");
                process::exit(1);
            }
            Ok(_) => {}
        }
    }
}

//################################
//             MAIN
//################################

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage : cargo run <Nombre de réponses type Informations personnelles>");
        process::exit(1);
    }

    let p_content = read_from_file("parrains.csv".to_string());
    let p_data = parse_csv(&p_content);

    let f_content = read_from_file("fillots.csv".to_string());
    let f_data = parse_csv(&f_content);

    let arg_decal = &args[1].parse::<i32>();
    let decal: usize; // Indice de fin des réponses Informations personnelles
    match arg_decal {
        Ok(val) => decal = *val as usize,
        Err(_) => {
            println!("Incorrect last argument");
            process::exit(1);
        }
    }

    let mut parrains = parse_parrains(&p_data, decal)[1..].to_vec();
    let mut fillots = parse_fillots(&f_data, decal)[1..].to_vec();

    calcul_parrains_pref(&parrains, &mut fillots);

    let fillots_par_parrains = (fillots.len() / parrains.len()) + 1;

    stable_matching(&mut parrains, fillots, fillots_par_parrains);

    write_results(parrains);
}
