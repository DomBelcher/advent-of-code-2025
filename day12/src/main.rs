use std::{collections::HashMap, fs};

const FILENAME: &str = "./input.txt";
const SHAPE_PART: char = '#';
const EMPTY_CHAR: char = '.';
const DIMENSION_CHAR: char = 'x';

// const PIECE_MODE: &str = "PIECES";
// const PUZZLE_MODE: &str = "PUZZLES";

#[derive(PartialEq, Eq)]
enum InputMode {
    PIECE_MODE,
    PUZZLE_MODE
}

fn main() {
    let (pieces, puzzles) = parse_input();

    for piece in pieces.values() {
        println!("Piece ID: {} dimensions: ({},{}), raw size: {}", piece.id, piece.width, piece.height, piece.raw_size);
    }

    let mut trivial_solution_count = 0;
    let mut indeterminate_count = 0;
    let mut trivially_valid = 0;

    for puzzle in puzzles {
        println!("Puzzle dimensions: ({},{}), raw size: {}", puzzle.width, puzzle.height, puzzle.width * puzzle.height);

        let sol = trivial_solution(&puzzle, &pieces);

        if sol.trivial_solution_exists {
            println!("Trivially solved: {}", sol.is_puzzle_solvable.unwrap());
            trivial_solution_count += 1;

            if sol.is_puzzle_solvable.unwrap() {
                trivially_valid += 1;
            }
        } else {
            println!("solution indeterminate");
            indeterminate_count += 1;
        }
    }

    println!("Trivially found answers for {} puzzles", trivial_solution_count);
    println!("Trivially {} puzzles are solvable", trivially_valid);
    println!("Unable to solve {} puzzles", indeterminate_count);
}

fn trivial_solution (puzzle: &Puzzle, pieces: &HashMap<usize, PuzzlePiece>) -> TrivialSolution {
    let puzzle_size = puzzle.width * puzzle.height;

    let mut min_size = 0;
    let mut max_size = 0;

    for (piece_id, count) in puzzle.pieces.iter() {
        let piece = pieces.get(piece_id).unwrap();
        min_size += piece.raw_size * count;
        max_size += (piece.width * piece.height) * count;
    }

    if max_size <= puzzle_size {
        // this may not guarantee a solution
        // as even though the puzzle has enough space for everything in a bounding box
        // it may not be possible to achieve that solution
        return TrivialSolution { trivial_solution_exists: true, is_puzzle_solvable: Some(true), solution_gap: None }
    }

    if min_size >= puzzle_size {
        // definitely not solvable
        // puzzle pieces have greater area than puzzle
        return TrivialSolution { trivial_solution_exists: true, is_puzzle_solvable: Some(false), solution_gap: None }
    }

    return TrivialSolution {
        trivial_solution_exists: false,
        is_puzzle_solvable: None,
        solution_gap: Some(max_size - puzzle_size) 
    }
}


fn parse_input () -> (HashMap<usize, PuzzlePiece>, Vec<Puzzle>) {
    let mut pieces = HashMap::new();
    let mut puzzles = vec![];

    let mut input_mode = InputMode::PIECE_MODE;

    let mut piece_id = 0;
    let mut piece_view = vec![];

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        if input_mode == InputMode::PIECE_MODE && line.split_whitespace().collect::<Vec<&str>>().len() > 1 {
            input_mode = InputMode::PUZZLE_MODE;
        }

        if input_mode == InputMode::PIECE_MODE {
            if line.len() == 0 {
                pieces.insert(piece_id, PuzzlePiece::from_input(piece_id, piece_view));
                piece_view = vec![];
                continue;
            }

            if line.contains(':') {
                piece_id = line.split(':').next().unwrap().parse::<usize>().unwrap();
                continue;
            }

            if line.contains(SHAPE_PART) || line.contains(EMPTY_CHAR) {
                piece_view.push(line);
                continue;
            }

            panic!("unparsable line: {}", line);
        }

        if input_mode == InputMode::PUZZLE_MODE {
            puzzles.push(Puzzle::from_input(line));
        }

    }

    return (pieces, puzzles)
}

struct Puzzle {
    width: usize,
    height: usize,
    pieces: HashMap<usize, usize>
}

impl Puzzle {
    fn from_input (input: &str) -> Puzzle {
        let sections = input.split(": ").collect::<Vec<&str>>();

        let dimensions = sections[0].split(DIMENSION_CHAR).map(|dim| dim.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let piece_sections = sections[1].split_whitespace().collect::<Vec<&str>>();

        let mut pieces = HashMap::new();
        for i in 0..piece_sections.len() {
            let piece_count = piece_sections[i].parse::<usize>().unwrap();
            pieces.insert(i, piece_count);
        }


        return Puzzle { width: dimensions[0], height: dimensions[1], pieces: pieces }
    }
}

struct TrivialSolution {
    trivial_solution_exists: bool,
    is_puzzle_solvable: Option<bool>,
    solution_gap: Option<usize> // difference between trivial max area of puzzle pieces and puzzle size
}

struct PuzzlePiece {
    raw_size: usize,
    width: usize,
    height: usize,
    id: usize,
    rotation: usize,
    reflection: bool,
    view: Vec<Vec<char>>
}

impl PuzzlePiece {
    fn from_input (id: usize, input: Vec<&str>) -> PuzzlePiece {
        let raw_size = input.iter().map(|row| row.chars().filter(|c| *c == SHAPE_PART).map(|_| 1_usize).sum::<usize>()).sum::<usize>();
        
        return PuzzlePiece {
            raw_size: raw_size,
            width: input[0].len(),
            height: input.len(),
            id: id,
            rotation: 0,
            reflection: false,
            view: input.into_iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<_>>()
        }
    }

    fn rotate (&self, angle: i32) -> PuzzlePiece {
        todo!("not implemented");

        return PuzzlePiece {
            raw_size: self.raw_size,
            width: 0,
            height: 0,
            id: self.id,
            rotation: 0,
            reflection: self.reflection,
            view: self.view
        }
    }
}