use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read};
use regex::Regex;


fn get_input(path: &String) -> Result<Vec<Vec<usize>>, String> {
    let path = Path::new(path);
    
    
    let file = match File::open(path)  {
        Ok(v) => v,
        Err(_) => {
            return Err(String::from("Could not open file!"))
        },
    };

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    _ = buf_reader.read_to_string(&mut contents);

    let mut parsed_input: Vec<Vec<usize>> = vec![];

    for line in contents.split("\n") {
        let re = Regex::new(r"\s+").unwrap();
        let split: Vec<usize> = re
            .split(line)
            .map( |s| s.parse::<usize>().unwrap_or(0))
            .collect();

        parsed_input.push(split);

    }

    //Assume at least two levels per report
    parsed_input = parsed_input.into_iter().filter(|vec| vec.len() > 2).collect();

    return Ok(parsed_input);
}


fn is_safe_report(report: &Vec<usize>) -> bool{
    let mut last = report[0];
    let mut increasing = true;
    if last > report[1] {
        increasing = false;
    }
    for i in 1..report.len() {
        let diff: i32 = (report[i] as i32) - (last as i32);
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        if diff < 0 && increasing {
            return false;
        }
        if diff > 0 && !increasing {
            return false
        }
        last = report[i]
    }
    return true;
}
#[derive(PartialEq)]
enum Gradient {
    Increasing,
    Decreasing,
    Even,
    FirstIter,
}

fn get_gradient(report: &Vec<usize>, comparator: i32, active: i32) -> (Gradient, i32) {
    if comparator == -1 {
        println!("Comparing {} and {}", -1, report[active as usize]);
        return (Gradient::FirstIter, 0 as i32);
    }
    println!("Comparing {} and {}", report[comparator as usize], report[active as usize]);

    let cur = report[active as usize] as i32;
    let prev = report[comparator as usize] as i32;
    let diff: i32 = cur - prev;
    if diff > 0 {
        return (Gradient::Increasing, diff);
    }
    if diff < 0 {
        return (Gradient::Decreasing, diff.abs());
    }
    return (Gradient::Even, 0);
}

fn is_problem(diff: usize, gradient: &Gradient, previous_gradient: &Gradient) -> bool {
    let mut problem = false;
    if diff < 1 || diff > 3 {
        problem = true;
    } 
    if *gradient != *previous_gradient && *previous_gradient != Gradient::FirstIter {
        problem = true;
    }
    return problem
}

fn is_safe_report_dampener(report: &Vec<usize>) -> bool{
    let mut prev_comp = -1;
    let mut comparator = -1;
    let mut active: i32;
    let mut available_skips = 1;

    let mut prev_gradient = Gradient::FirstIter;
    //TODO Special case for last?
    let mut i: usize = 0;
    let end: usize = report.len();
    loop{

        active = i as i32;
        let (gradient, diff) = get_gradient(&report, comparator, active);
        if gradient == Gradient::FirstIter {
            prev_comp = comparator;
            comparator = active;
        } else {
            
            let problem = is_problem(diff as usize, &gradient, &prev_gradient);

            if problem && available_skips > 0 {
                available_skips -= 1;

                //TODO Either change Comparator to previous one (previous level is the problem)
                //  Or leave comparator as is and go to next iteration (active level is the problem)
                if gradient == Gradient::FirstIter {
                    comparator = prev_comp;
                    continue;
                }
            } else if problem {
                //Skip a first wrong element
                if comparator == 0 {
                    comparator = 1;
                    continue;
                }
                return false;
            } else {
                prev_comp = comparator;
                comparator = active;
                prev_gradient = gradient;
            }
        }


        //Loop stuff
        i += 1;
        if i == end {
            break;
        }
    }

    return true;
}

fn main() {
    let path = String::from("input.txt");

    let input = match get_input(&path) {
        Ok(v)=> v,
        Err(e) => {
            print!("{}", e);
            return;
        }
    };
    let mut sum_safe_reports = 0;
    for report in &input {
        //print!("{:?}", report);
        if is_safe_report(&report) {
            sum_safe_reports += 1;
        }
    }

    print!("Number of safe reports: {}\n", sum_safe_reports);

    let mut sum_safe_reports_dampener = 0;
    for report in &input {
        if is_safe_report_dampener(&report) {
            sum_safe_reports_dampener += 1;
            println!("---");

        } else {
            println!("{:?}", report);
        }
    }
    print!("Number of safe reports with the Problem Dampener: {}\n", sum_safe_reports_dampener)

}
