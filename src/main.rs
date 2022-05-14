fn main() {
    let a = ["A partridge in a pear tree", "Two turtle doves, and", "Three french hens", "Four calling birds", "Five golden rings", "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth", "tenth", "eleventh", "twelfth"];

    for i in 0..12 {
        //println!("This element is: {}", a[i]);
        println!("On the {} day of Christmas, my true love sent to me", days[i]);
        let index = i;
        for j in (0..index+1).rev() {
            println!("{}",a[j]);
        }
        println!("\n");
    }
}
