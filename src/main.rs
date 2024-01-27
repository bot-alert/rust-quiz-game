use std::io;

fn main() {
    let file = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("problem.csv");


    match file {
        Ok(mut content) => {
            let mut total_question: f32 = 0.0;
            let mut correct_ans: f32 = 0.0;

            content.records()
                .filter_map(|record| record.ok())
                .for_each(|record| {
                    if let (Some(question), Some(answer)) = (record.get(0), record.get(1)) {
                        let mut input = String::new();
                        println!("What is the sum of {} ", question);
                        loop {
                            if let Ok(_) = io::stdin().read_line(&mut input) {
                                break;
                            }
                            println!("Cannot read the input, please try again");
                            _ = io::stdin()
                        }
                        if input.trim() == answer {
                            correct_ans = correct_ans + 1.0;
                        }
                    }
                    total_question = total_question + 1.0;
                });


            println!("Total question {} Correct ans {}", total_question, correct_ans);

            println!("Your accuracy is {} %", (correct_ans / total_question) * 100.0)
        }
        Err(_) => {
            println!("Cannot read the csv file");
        }
    }
}