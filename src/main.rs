extern crate rand;

use rand::Rng;

/// A Rust implementation of a CIS 22a final project my partner did/is doing:
///
/// > Write a program to simulate the game with 10,000 trials and show that the
/// > chance of winning is 66.67% if the player chose to switch (doors), every
/// > time.
/// >
/// > The simulation code should carry out every step of the game:
/// >   a. Place, randomly, the two goats and the car behind two doors;
/// >   b. Contestant picks the door;
/// >   c. The host opens one door with a goat behind;
/// >   d. The contestant switches to the other closed door;
/// >   e. Determine if contestant wins or loses
/// >   f. Report the number of winners;
/// >   g. Output the ration with empirical probability
/// >
/// > You shouldn't use the findings in the first approach above to just compute
/// > the mathematical results (e.g. this is expected to be a simulation not a
/// > a mathematical proof).
///
/// # Notes
///
/// I am limiting myself to basic C++-relative primites from the CIS 22a class
/// (e.g. booleans, ints, arrays, strings) with the exception of using enums -
/// because I think that makes the code significantly more readable without
/// really violating that principal.
fn main() {
  let times = 10000;
  let results = run(times);
  let percent = (f32::from(results) / f32::from(times)) * 100.0;
  println!("Wins: {} (of 10000); {1:.2}% of the time", results, percent)
}

/// Runs the specified amount of simulations returning the number of winners.
fn run(times: u16) -> u16 {
  let mut winners: u16 = 0;
  let mut rng = rand::thread_rng();

  for _ in 0..times {
    if simulate(&mut rng) {
      winners += 1;
    }
  }

  winners
}

/// Runs a simulation and returns whether the contestant succeeded or not.
fn simulate(rng: &mut rand::rngs::ThreadRng) -> bool {
  // A: Generate 3 doors, and place a car behind one of them.
  let mut doors: [Door; 3] = [Door::Goat, Door::Goat, Door::Goat];
  let car = rng.gen_range(0, 3);
  doors[car] = Door::Car;

  // B: Assume the contestant always picks door 1 [i=0].
  // No code necessary, we just don't even consider doors[0].

  // C: Host should open either door 2 (if it is a goat) or 3 (otherwise).
  // D: What that really means is the contestant "picks" the other door.
  let switch = match doors[1] {
    // Host reveals "door 2 is a goat", what would you like to do?
    // Contestant switches to door 3 [i=2].
    Door::Goat => &doors[2],

    // Host reveals "door 3 is goat", what would you like to do?
    // Contestant switches to door 2 [i=1].
    Door::Car => &doors[1],
  };

  // E: Deterime/return if the contestant won (got a car) or lost (got a goat).
  match switch {
    Door::Goat => false,
    Door::Car => true,
  }
}

/// Possible results behind a door.
enum Door {
  Car,
  Goat,
}
