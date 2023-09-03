use rand::Rng;

// snakes and ladders board, '0' represents an empty space
// any other number is where that snake or ladder will put you.
const BOARD: [u8; 100] = [
    0, 0, 0, 14, 0, 0, 0, 0, 31, 0,
    0, 0, 0, 0, 0, 0, 7, 0, 0, 0,
    42, 0, 0, 0, 0, 0, 0, 84, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    67, 0, 0, 34, 0, 0, 0, 0, 0, 0,
    0, 19, 0, 60, 0, 0, 0, 0, 0, 0,
    0, 91, 0, 0, 0, 0, 0, 0, 0, 99,
    0, 0, 0, 0, 0, 0, 36, 0, 0, 0,
    0, 0, 73, 0, 75, 0, 0, 79, 0, 0];

fn turn(mut space: u8) -> u8 {
    let mut rng = rand::thread_rng();

    let roll: u8 = rng.gen_range(1..7); // rolls the dice (random number 1-6)
    space += roll; // adds rolls to current space

    // outputs the current space and roll to the user
    println!("You rolled a {}.", roll); 
    println!("You are now on space {}.", space);

    // returns 'space'
    space
}

fn check_over_goal(mut space: u8) -> u8 {
    if space > 100 { // checks if the player has gone over the goal
        let spaces_over = space - 100; // gets how many spaces the user has gone over 100
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
    let space_index: usize = (space - 1).into(); 

    if BOARD[space_index] != 0 {
        let mut thing = ""; // thing is either a snake or a ladder
        if BOARD[space_index] > space { // if space is smaller than the index of 'BOARD', it's a ladder
            thing = "ladder";
        } else if BOARD[space_index] < space {
            thing = "snake"; // if space is bigger than the index of 'BOARD', it's a snake
        }
        
        space = BOARD[space_index]; // sets space to the end of the object (snake or ladder)

        // outputs the current space and the object to the user
        println!("You have landed on a {}.", thing); 
        println!("You are now on space {}.", space);
    }

    // returns 'space'
    space
}

fn main() {
    // define variables
    let mut space: u8 = 1;
    let mut turns: u16 = 0;
    let mut play: bool = true;

    // print name of game
    println!("Snakes & Ladders");

    // code loop
    while play {
        // logic for inputs
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("");
        input = input.trim().to_owned(); // trims whitespace of front and end of input
        if input.len() == 1 && input == "q" { // if input == q, quit program
            return;
        }

        turns += 1;

        // play 1 turn
        space = turn(space);

        // check if the user has won
        play = space != 100;

        space = check_over_goal(space);
        space = check_movement(space);
    }

    // tells the user they've won and how many turns it took.
    println!("\nYou win!!!\nIt took {} turns.", turns);
}
