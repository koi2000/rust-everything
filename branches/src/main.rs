fn sif() {
    let number = 3;
    // if 语句后面的值只能是bool类型
    if number < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }
}

fn selseif() {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn letif() {
    let condition = true;
    let number = if condition { 1 } else { 6 };
    // 这种会报错，以为分支必须为同一类型
    // let number = if condition { 5 } else { "six" };
    println!("The value is {number}");
}

fn loopfn() {
    loop {
        println!("looping.....")
    }
}

fn loopbreak() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}

fn breaklabel() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn whileloop() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn forloop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is {}", a[index]);
        index += 1;
    }
}

fn enhancefor() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn reverse() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn main() {
    sif();
}
