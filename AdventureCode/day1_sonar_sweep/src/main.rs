
fn main() {
        
    let path: &str = "sonar_messures.txt";
    let incial_readings: String = std::fs::read_to_string(path).expect("cannot read readings");
    let mut fin_reading: Vec<String> = Vec::new();
    
    for read in incial_readings.lines() {
        fin_reading.push(read.to_owned());
    }

    //part_one(fin_reading);

    //part_two(fin_reading);
}


fn part_one(fin_reading: Vec<String>) {


    let mut count_ocours: u64 = 1;
    let mut count_no_ocours: u64 = 0;
    let mut analized: u64 = 0;

    println!("{}  => FIRST READING", fin_reading[0]);

    for i in 1..fin_reading.len() {

        analized = analized + 1;

        if i <= fin_reading.len() {
            let current = fin_reading[i - 1].to_owned();
            let next = fin_reading[i].to_owned();

            if current < next {
                //println!("{} > {}  => (increased)", next, current);
                count_ocours = count_ocours + 1;

            } else if current > next {
                //println!("{} => (decreased)", next);
                count_no_ocours = count_no_ocours + 1;
            }

        }
    }

    println!("{}  => TOTAL INCREASED READINGS", count_ocours);

    println!("{}  => TOTAL DECREASED READINGS", count_no_ocours);

    println!("{}  => TOTAL ANALIZED READINGS", analized);

    println!("{}  => TOTAL READINGS", fin_reading.len());
}

fn part_two(fin_reading: Vec<String>) {

    let mut count_ocours = 0;
    let mut reads = 0;

    let mut i = 0;
    let mut next_i = i + 1;

    while next_i < fin_reading.len() - 1 {

        if next_i + 2 >= fin_reading.len() || i + 2 >= fin_reading.len() {
            break;
        }

        let c_n1 = fin_reading[i].parse::<i32>().unwrap();
        let c_n2 = fin_reading[i + 1].parse::<i32>().unwrap();
        let c_n3 = fin_reading[i + 2].parse::<i32>().unwrap();
        
        let n_n1 = fin_reading[next_i].parse::<i32>().unwrap();
        let n_n2 = fin_reading[next_i + 1].parse::<i32>().unwrap();
        let n_n3 = fin_reading[next_i + 2].parse::<i32>().unwrap();

        let sum_c = c_n1 + c_n2 + c_n3;
        let sum_n = n_n1 + n_n2 + n_n3;
        
        if sum_c < sum_n {
            println!("{c_n1} + {c_n2} + {c_n3} = {sum_c} > {n_n1} + {n_n2} + {n_n3} = {sum_n}");
            count_ocours = count_ocours + 1;
        }

        i = i + 1;
        next_i = i + 1;
        reads = reads + 1;
    }



    println!("{} TOTAL READS", reads);
    println!("{} TOTAL INCREASED", count_ocours);
}