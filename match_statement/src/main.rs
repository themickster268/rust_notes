mod matching_option;
mod if_let;


/* #[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
    Arizona,
    Arkinsas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    TexasUtah,
    Vermont,
    Virginia,
    Washingtion,
    WestVirginia,
    Wisonsin,
    Wyoming
}

enum Coin {
    Penny, 
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
} */


/*
  let mut count = 0;
  match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}", state),
    _ => count +=1
  }
 */

/*
  let mut counter = 0;

  if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}", state);
  } else {
    count += 1;
  }
*/

fn main() {
    // matching_option::run();
    if_let::run();

}
