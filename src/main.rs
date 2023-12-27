use rand::Rng;

// snakes and ladders board, '0' represents an empty space
// any other number is where that snake or ladder will put you.
const BOARD: [u8; 100] = [
    0, 0, 0, 14, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 84, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 67, 0, 0, 34, 0, 0, 0, 0, 0, 0, 0,
    19, 0, 60, 0, 0, 0, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 0, 0, 99, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0,
    0, 73, 0, 75, 0, 0, 79, 0, 0,
];

const GOAL: u8 = 100; // goal space
const NO_PLAYERS: i8 = 2;

struct Player {
    current_space: u8, // current space the player is on
    turns: u8,         // number of turns the player has taken
    won: bool,         // whether the player has won the game or not
    name: String,      // name of the player
    moves: Vec<u8>,    // moves taken by the player
}

impl Player {
    // creates a new player
    fn new(name: String) -> Player {
        Player {
            current_space: 1,
            turns: 0,
            won: false,
            name,
            moves: Vec::new(),
        }
    }

    fn clone(&self) -> Player {
        Player {
            current_space: self.current_space,
            turns: self.turns,
            won: self.won,
            name: self.name.clone(),
            moves: self.moves.clone(),
        }
    }
}

fn turn(mut player: Player) -> Player {
    player.turns += 1;

    let name = player.name.clone();

    // play 1 turn
    player.current_space = roll(player.current_space, name);

    // check if the user has won
    // play = player_1.current_space != GOAL;

    player.current_space = check_over_goal(player.current_space);
    player.current_space = check_movement(player.current_space);

    player.moves.push(player.current_space);

    player
}

fn roll(mut space: u8, name: String) -> u8 {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    let roll: u8 = rng.gen_range(1..7); // rolls the dice (random number 1-6)
    space += roll; // adds rolls to current space

    // outputs the current space and roll to the user
    println!("{} rolled a {}.", name, roll);
    println!("You are now on space {}.", space);

    // returns 'space'
    space
}

fn check_over_goal(mut space: u8) -> u8 {
    if space > 100 {
        // checks if the player has gone over the goal
        let spaces_over: u8 = space - GOAL; // gets how many spaces the user has gone over 100
        space -= spaces_over * 2; // decreases current spaces by 'spaces over 100' * 2 (*2 is necessary to reduce the space below 100)

        // outputs the current space and how many spaces over 100 to the user
        println!("You have rolled {} spaces over 100.", spaces_over);
        println!("You are now on space {}.", space);
    }

    // returns 'space'
    space
}

fn check_movement(mut space: u8) -> u8 {
    // converts the space into an index and 'usize' type
    let space_index: usize = (space - 1) as usize;

    if BOARD[space_index] != 0 {
        let movement_object: &str;

        if BOARD[space_index] > space {
            movement_object = "ladder";
        } else {
            movement_object = "snake";
        }

        space = BOARD[space_index]; // sets space to the end of the object (snake or ladder)

        // outputs the current space and the object to the user
        println!("You have landed on a {}.", movement_object);
        println!("You are now on space {}.", space);
    }

    // returns 'space'
    space
}

fn main() {
    // define variables
    // let mut play: bool = true;

    let mut players: Vec<Player> = Vec::new();

    println!("Snakes & Ladders\n");
    for i in 0..NO_PLAYERS {
        // creates new players
        println!("\nEnter name of player {}:", i + 1);

        let mut name = String::new();
        std::io::stdin().read_line(&mut name).expect("");
        name = name.trim().to_owned(); // trims whitespace of front and end of input
        players.push(Player::new(name));
    }

    let mut current_player: i8 = 1;

    // print name of game

    // code loop
    loop {
        // logic for inputs
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).expect("");
        input = input.trim().to_owned(); // trims whitespace of front and end of input
        if input.len() == 1 && input == "q" {
            // if input == q, quit program
            return;
        }

        let current_player_index: usize = (current_player - 1) as usize;

        // logic for turn
        let player: Player = players[current_player_index].clone();
        players[current_player_index] = turn(player);

        players[current_player_index].won = players[current_player_index].current_space == GOAL;

        if players[current_player_index].won {
            // tells the user they've won and how many turns it took.
            println!(
                "\n{} wins!!!\nIt took {} turns.",
                players[current_player_index].name, players[current_player_index].turns
            );
            return;
        }

        current_player += 1;
        if current_player > NO_PLAYERS {
            current_player = 1;
        }
    }
}
