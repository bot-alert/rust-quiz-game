use std::io;

fn main() {
    let mut file = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("problem.csv")
        .unwrap_or_else(|e| panic!("Cannot read the csv file"));


    let mut total_question: f32 = 0.0;
    let mut correct_ans: f32 = 0.0;

    file.records()
        .filter_map(|record| record.ok())
        .for_each(|record| {
            if let (Some(question), Some(answer)) = (record.get(0), record.get(1)) {
                let mut input = String::new();
                println!("What is the sum of {} ", question);

                while io::stdin().read_line(&mut input).is_err() {
                    println!("Cannot read the input, please try again");
                }

                if input.trim() == answer {
                    correct_ans += 1.0;
                }
            }
            total_question += 1.0;
        });


    println!("Total question {} Correct ans {}", total_question, correct_ans);

    println!("Your accuracy is {} %", (correct_ans / total_question) * 100.0)
}