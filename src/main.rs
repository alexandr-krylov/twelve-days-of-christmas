use std::ops::Index;

fn main() {
    let order = ["first", "second",
     "third", "fourth", "fifth",
     "sixth", "seventh", "eighth",
     "ninth", "tenth", "eleventh",
     "twelfth"];
    let gifts = ["A partridge in a pear tree", "Two turtle doves,",
     "Three French hens", "Four calling birds", "Five golden rings",
     "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking",
     "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping",
     "Twelve drummers drumming"];
    for number_love in order.iter() {
        println!("{},{:#?}", number_love, order.iter);
    }
}
