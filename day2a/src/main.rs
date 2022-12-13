use std::fs::read_to_string;

fn read_input() -> String {
    // Read the file.
    match read_to_string("input.txt") {
        Ok(value) => return value,
        Err(why) => panic!("{}", why),
    }
}

fn score_game(opponents_play: char, my_play: char) -> i32 {
    let shape_score = match my_play {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("Unexpected play {}", my_play),
    };
    let win_score = match opponents_play {
        // They throw rock.
        'A' => match my_play {
            'X' => 3,  // I throw rock, tie.
            'Y' => 6,  // I throw paper, win.
            'Z' => 0,  // I throw scissors, lose.
            _ => panic!("Unexpected play {}", my_play),
        },
        // They throw paper.
        'B' => match my_play {
            'X' => 0,  // I throw rock, lose.
            'Y' => 3,  // I throw paper, tie.
            'Z' => 6,  // I throw scissors, win.
            _ => panic!("Unexpected play {}", my_play),
        },
        // They throw scissors.
        'C' => match my_play {
            'X' => 6,  // I throw rock, win.
            'Y' => 0,  // I throw paper, lose.
            'Z' => 3,  // I throw scissors, tie.
            _ => panic!("Unexpected play {}", my_play),
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
        let my_play = chars[2];
        total += score_game(opponents_play, my_play);
    }
    println!("{}", total);
}
