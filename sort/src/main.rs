fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut nums: Vec<i32> = vec![];

    for arg in &args
    {
        let n= arg.parse::<i32>();
        match n {
            Ok(i) => nums.push(i),
            _ => ()
        }
    }
    // dbg!(&nums);

    for (i, n) in nums.iter().enumerate()
    {
        if i > 10
        {
            print!("...");
            break;
        }
        print!("{}, ", n);
    }
    println!();
    let now = std::time::Instant::now();
    let sorted = sort(&nums);
    let elapsed = now.elapsed();

    println!("time to run sort: {:.2?}", elapsed);

    for (i, n) in sorted.iter().enumerate()
    {
        if i > 10
        {
            print!("...");
            break;
        }
        print!("{}, ", n);
    }
    println!();
    // dbg!(_sorted);
}

fn pair_nums(nums: &Vec<i32>) -> Vec<(i32, i32)>
{
    let mut pairs: Vec<(i32, i32)> = vec![];
    let mut prev: i32 = 0;
    for (i, n) in nums.iter().enumerate()
    {
        if i % 2 == 0
        {
            prev = *n;
            continue;
        }

        let max: i32 = if *n > prev { *n } else { prev };
        let min: i32 = if *n < prev { *n } else { prev };

        let pair: (i32, i32) = (min, max);

        pairs.push(pair);
        prev = *n;
    }
    if nums.len() > 0 && nums.len() % 2 == 1
    {
        pairs.push((*nums.last().unwrap(), *nums.last().unwrap()));
    }
    // dbg!(&pairs);
    pairs
}

fn sort(nums: &Vec<i32>) -> Vec<i32>
{
    if nums.len() == 1 || nums.len() == 0
    {
        return nums.clone();
    }
    let mut bigger: Vec<i32> = vec![];
    let pairs = pair_nums(nums);

    for pair in &pairs
    {
        if pair.0 == pair.1
        {
            continue;
        }
        bigger.push(pair.1);
    }
    bigger = sort(&bigger);

    for pair in &pairs
    {
        match bigger.binary_search(&pair.0) {
            Err(e) => bigger.insert(e, pair.0),
            _ => ()
        }
    }
    bigger
}