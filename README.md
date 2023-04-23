# Prancing Pony

*Even from the outside the inn looked a pleasant home to familiar
eyes.  A sign above the door pictured a fat white pony rearing on its
hind legs.  Over the door was painted the letters:  _THE PRANCING PONY by
BARLIMAN BUTTERBALL_.*

Prancing Pony is a simple implementation of the Knight's Tour using the backtracking strategy for variable board sizes and variable starting positions. It is guaranteed to find a solution and will know immediately if the given parameters have no possible Tour. 

Because this is a brute force and naive *O(n!)* solution, it should **NOT** be expected to find a result in the given parameters in most configurations, however it *should* behave well for small boards (less than 40 squares) or traditional boards with 8x8 size starting at position (0,0).

## Usage

```
let tour_input = prancing_pony::TourInput {
    size_x: 8,
    size_y: 8,
    starting_position: (0,0)
};

let result = prancing_pony::find_solution(tour_input);

match result {
    prancing_pony::TourResult::Solution(t) => {
        println!("Solution found in {} nanoseconds after {} backtracks", t.calculation_time, t.times_backtracked);
        for (index, position) in t.position_history.iter().enumerate() {
            println!("{} - ({} {})", index, position.0, position.1);
        }
    },
    prancing_pony::TourResult::NoSolution => {  println!("No Solution"); },
    prancing_pony::TourResult::InvalidParameters => { println!("Invalid Parameters"); }
}
```
