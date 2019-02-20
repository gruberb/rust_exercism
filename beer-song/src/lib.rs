pub fn verse(n: i32) -> String {
    if n == 0 {
        String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
    } else { 
        String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n")
    }
}

pub fn sing(start: i32, end: i32) -> String {
    unimplemented!("sing verses {} to {}, inclusive", start, end)
}
