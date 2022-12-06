use std::fs;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors
}
impl Move {
    fn from_string(string: &str) -> Move {
        match string{
            "X" | "A" => Move::Rock,
            "Y" | "B" => Move::Paper,
            "Z" | "C" => Move::Scissors,
            _ => panic!("Not a valid character for move parsing")
        }
    }
    fn to_points(&self) -> i32 {
        match self{
            &Move::Rock => 1,
            &Move::Paper => 2,
            &Move::Scissors => 3,
        }
    }
    fn get_required_move(enemy_move: &Move, desired_outcome: &Outcome) -> Move {
        match desired_outcome {
            &Outcome::Win => match enemy_move {
                &Move::Rock => Move::Paper,
                &Move::Paper => Move::Scissors,
                &Move::Scissors => Move::Rock
            },
            &Outcome::Tie => enemy_move.clone(),
            &Outcome::Loss => match enemy_move {
                &Move::Rock => Move::Scissors,
                &Move::Paper => Move::Rock,
                &Move::Scissors => Move::Paper
            }
        }
    }
}

enum Outcome {
    Win,
    Loss,
    Tie
}
impl Outcome {
    fn from_string(string: &str) -> Outcome {
        match string {
            "X" => Outcome::Loss,
            "Y" => Outcome::Tie,
            "Z" => Outcome::Win,
            _ => {
                    println!("Couldn't match character \'{}\'", string);
                    panic!()
                }
            }
    }
    fn calculate(enemy_move: &Move, my_move: &Move) -> Outcome {
        match enemy_move {
            &Move::Rock => match my_move {
                &Move::Rock => Outcome::Tie,
                &Move::Paper => Outcome::Win,
                &Move::Scissors => Outcome::Loss,
            },
            &Move::Paper => match my_move {
                &Move::Rock => Outcome::Loss,
                &Move::Paper => Outcome::Tie,
                &Move::Scissors => Outcome::Win,
            },
            &Move::Scissors => match my_move {
                &Move::Rock => Outcome::Win,
                &Move::Paper => Outcome::Loss,
                &Move::Scissors => Outcome::Tie
            },
        }
    }
    fn to_points(&self) -> i32 {
        match self {
            &Outcome::Loss => 0,
            &Outcome::Tie => 3,
            &Outcome::Win => 6,
        }
    }
}

fn main() {
    let lines = fs::read_to_string("input.txt").expect("File not found");
    let mut p1_total: i32 = 0;
    let mut p2_total: i32 = 0; 

    for line in lines.split("\r\n") {
        let moves: Vec<&str> = line.split(' ').collect();
        let enemy_move = Move::from_string(moves.get(0).unwrap());
        let my_move = Move::from_string(moves.get(1).unwrap());
        let outcome = Outcome::calculate(&enemy_move, &my_move);
        p1_total += my_move.to_points() + outcome.to_points();

        let desired_outcome = Outcome::from_string(moves.get(1).unwrap());
        let needed_move = Move::get_required_move(&enemy_move, &desired_outcome);
        p2_total += needed_move.to_points() + desired_outcome.to_points();
    }
    dbg!((p1_total, p2_total));
}
