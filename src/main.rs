fn main() {
    let order = ["first", "second",
     "third", "fourth", "fifth",
     "sixth", "seventh", "eighth",
     "ninth", "tenth", "eleventh",
     "twelfth"];
    let gifts = ["A partridge in a pear tree.", "Two turtle doves,",
     "Three French hens,", "Four calling birds,", "Five golden rings,",
     "Six geese a-laying,", "Seven swans a-swimming,", "Eight maids a-milking,",
     "Nine ladies dancing,", "Ten lords a-leaping,", "Eleven pipers piping,",
     "Twelve drummers drumming,"];
    let mut order_index = 0;
    println!("Twelve Days of Christmas");
    println!();
    while order_index < order.len() {
        println!("On the {} day of Christmas,", order[order_index]);
        println!("my true love sent to me");
        let mut gifts_index = order_index + 1;
        while gifts_index > 0 {
            println!("{}", gifts[gifts_index - 1]);
            gifts_index = gifts_index - 1;
        }
        order_index = order_index + 1;
        println!();
    }
}
