use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::vec;
const CARGO_TOML_PATH: &str = "./minefield.txt";

/// Imprime por pantalla y guarda en un archivo el resultado del campo de juego, ó el error correspondiente.
///
/// file: Contiene el archivo principal con el campo inicial.
///
/// v: Vector de vectores char para analizar.
///
/// for loops: Analiza paso a paso el archivo campo de juego para luego analizar
/// donde hay minas y cuantas hay al rededor de un punto. En caso de encontrar
/// una linea totalmente en blanco, la ignora.
pub fn run() -> Result<(), Box<dyn Error>> {
    let file = File::open(CARGO_TOML_PATH)?;
    let mut v: Vec<Vec<char>> = vec![];
    let buffered_reader = BufReader::new(file);

    for line_result in buffered_reader.lines() {
        let value = line_result?; // guardo el resultado de usar ?
        if value.is_empty() {
            continue;
        }
        let mut int_lines: Vec<char> = vec![];
        for c in value.chars() {
            int_lines.push(c);
        }
        v.push(int_lines)
    }
    let minefield: Vec<String> = field_analyzer(v);

    let mut out_file = File::create("./mines_output.txt").expect("Unable to create file");
    let eol = "\n";
    for field_lines in minefield {
        println!("{}", &field_lines);
        let value = field_lines + eol;
        out_file
            .write_all(value.as_bytes())
            .expect("Unable to write data");
    }

    Ok(())
}

/// Este metodo devuelve el resultado del campo de juego.
///
/// Indica las minas, la cantidad de minas adyacentes a un espacio vacío, y en caso de
/// no tener minas al rededor devolverá el mismo espacio vacío.
///
/// mine_line: Fila del tablero de juego con ".", "num" o "*".
///
/// minefield: Tablero de juego procesado con todas sus mine_line's.
pub fn field_analyzer(vect: Vec<Vec<char>>) -> Vec<String> {
    let mut minefield: Vec<String> = vec![];
    for (i, value) in vect.iter().enumerate() {
        let mut mine_line = String::from("");
        for (j, value) in value.iter().enumerate() {
            if *value == '*' {
                mine_line.push(*value);
                continue;
            } else {
                let mine_num: u32 = metal_detector(i, j, &vect);
                if mine_num == 0 {
                    mine_line.push('.');
                } else {
                    mine_line.push_str(&mine_num.to_string());
                }
            }
        }
        minefield.push(mine_line);
    }
    minefield
}

///
/// Este metodo devuelve la cantidad de minas vecinas a un punto como un dato u32.
///
/// iflag y jflag: Check de overflow para sumas y restas de los ejes i y j, la posicón central queda
/// hardcodeada en false ya que no es posible tener overflow consigo mismo.
///
/// mine_counter: Contador de minas vecinas.
///
/// for loops*: Recorro las 8 casillas vecinas siempre y cuando no hayan saltado los flags
/// de overflow.
///
/// *Este metodo checkea contra si mismo pero lo prefiero por su simpleza de lectura de codigo.
pub fn metal_detector(i: usize, j: usize, vect: &Vec<Vec<char>>) -> u32 {
    let iflags: Vec<bool> = vec![i.overflowing_sub(1).1, false, i + 2 > vect.len()];
    let jflags: Vec<bool> = vec![j.overflowing_sub(1).1, false, j + 2 > vect[i].len()];
    let mut mine_counter = 0;

    for iop in iflags.iter().enumerate() {
        for jop in jflags.iter().enumerate() {
            if !iop.1 && !jop.1 && vect[(i + iop.0) - 1][(j + jop.0) - 1] == '*' {
                mine_counter += 1;
            }
        }
    }

    mine_counter
}

//pub struct Config {
//    pub filename: String,
//}
//
//impl Config {
//    pub fn new(args: &[String]) -> Result<Config, &str> {
//        if args.len() < 2 {
//            return Err("Missing arguments!");
//        }
//
//        let filename = args[1].to_string();
//
//        Ok(Config { filename })
//    }
//}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn no_mines() {
//        let query = "duct";
//        let contents = "\
//Rust:
//safe, fast, productive.
//pick three.";
//
//        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
//    }
//}
