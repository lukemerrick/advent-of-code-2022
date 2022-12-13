use std::fs::read_to_string;

fn read_input() -> String {
    // Read the file.
    match read_to_string("input.txt") {
        Ok(value) => return value,
        Err(why) => panic!("{}", why),
    }
}

fn score_game(opponents_play: char, outcome: char) -> i32 {
    let win_score = match outcome {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => panic!("Unexpected outcome {}", outcome),
    };
    let shape_score = match opponents_play {
        // They throw rock.
        'A' => match outcome {
            'X' => 3,  // I throw scissors to lose.
            'Y' => 1,  // I throw rock to tie.
            'Z' => 2,  // I throw paper to win.
            _ => panic!("Unexpected outcome {}", outcome),
        },
        // They throw paper.
        'B' => match outcome {
            'X' => 1,  // I throw rock to lose.
            'Y' => 2,  // I throw paper to tie.
            'Z' => 3,  // I throw scissors to win.
            _ => panic!("Unexpected outcome {}", outcome),
        },
        // They throw scissors.
        'C' => match outcome {
            'X' => 2,  // I throw paper to lose.
            'Y' => 3,  // I throw scissors to tie.
            'Z' => 1,  // I throw rock to win.
            _ => panic!("Unexpected outcome {}", outcome),
        },
        _ => panic!("Unexpected opponents play {}", opponents_play),
    };
    return win_score + shape_score;
}

fn main() {
    let mut total: i32 = 0;
    for line in read_input().trim().split("\n") {
        let chars: Vec<char> = line.chars().collect();
        assert_eq!(chars[1], ' ');
        let opponents_play = chars[0];
        let outcome = chars[2];
        total += score_game(opponents_play, outcome);
    }
    println!("{}", total);
}
