const CARD_PK: usize = 335121;
const DOOR_PK: usize = 363891;
const DIV: usize = 20201227;

fn main() {
    println!("Hello, world!");

    let subject_number_card = 7;
    let subject_number_door = DOOR_PK;
    let (mut key_card, mut key_door) = (1, 1);
    for _ in 1..DIV {
        if key_card == CARD_PK {
            break;
        }
        key_card = step(key_card, subject_number_card);
        key_door = step(key_door, subject_number_door);
    }

    println!("Part 1: {}", key_door)
}

fn step(i: usize, subject_number: usize) -> usize {
    (i * subject_number) % DIV
}
