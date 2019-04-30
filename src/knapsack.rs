extern crate rand;

use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;


#[derive(Clone, Debug)]
struct Item {
    name: String,
    weight: usize,
    value: usize,
}

#[derive(Clone, Debug)]
struct Knapsack<'a> {
    value: usize,
    contents: Option<Vec<&'a Item>>,
}

impl<'a> Knapsack<'a> {
    fn empty() -> Self {
        Knapsack { value: 0, contents: None }
    }
}

fn make_table(items: &Vec<Item>, capacity: usize, band_size: usize) -> Knapsack {

    let mut table: Vec<Vec<Knapsack>> = vec![];
    let mut optimal = Knapsack::empty();

    // cycle through items
    for (i, item) in items.iter().enumerate() {

        // add a row to the table
        let mut row: Vec<Knapsack> = vec![];
        let weight = item.weight;
        let value = item.value;

        // for each object, cycle through weight bands
        for j in 0..((capacity / band_size)) {
            let weight_limit = (j + 1) * band_size;

            // get previous best knapsack for this weight
            let prev_max = match i {
                0 => Knapsack::empty(), // new empty Knapsack
                _ => table[i - 1][j].clone(),
            };

            if weight > weight_limit {
                // can't use this item: use previous max value
                row.push(prev_max);

            } else {
                // calculate value of remaining space
                let weight_remaining = weight_limit - weight;
                let sub_knapsack = {
                    if weight_remaining >= band_size {
                        let weight_column = (weight_remaining / band_size) - 1;
                        match i {
                            0 => Knapsack::empty(),
                            _ => table[i - 1][weight_column].clone(), // max from prev row for weight available
                        }
                    } else {
                        Knapsack::empty()
                    }
                };

                // now compare value + value of sub-knapsack with previous max value

                // option 1: sub-knapsack has items and value
                if let Some(c) = sub_knapsack.contents {
                    let total_value = value + sub_knapsack.value;
                    if total_value > prev_max.value {
                        let mut new_contents = c.clone();
                        new_contents.push(&item);
                        let knapsack = Knapsack { value: total_value, contents: Some(new_contents) };
                        row.push(knapsack.clone());
                        optimal = knapsack;
                    } else {
                        row.push(prev_max);
                    }

                    // option 2: sub-knapsack is empty
                } else if value > prev_max.value {
                    let knapsack = Knapsack { value: value, contents: Some(vec![item]) };
                    row.push(knapsack.clone());
                    optimal = knapsack;
                } else {
                    row.push(prev_max);
                }
            }
        }
        table.push(row);

    }
    optimal
}

fn generate_items(n: usize) -> Vec<Item> {
    let lemon = Item { name: "lemon".to_string(), weight: 50, value: 1} ;
    let sugar = Item {name: "sugar".to_string(), weight:500, value: 2} ;
    let book = Item {name: "book".to_string(), weight: 150, value: 6} ;
    let wine = Item {name: "wine".to_string(), weight: 400, value: 12 } ;
    let all_items = vec![lemon, sugar, book, wine];
    let mut rng = thread_rng();
    let sampled: Vec<Item> = (0..n).map(|_|
        all_items[rng.gen_range(0, all_items.len())].clone())
        .collect();
    sampled
}

pub fn run() {
    let random_items = generate_items(20);
    for item in &random_items {
        println!("{:?}", item);
    }

    let optimal = make_table(&random_items, 1750, 50);
    for item in optimal.contents.unwrap() {
        println!("optimal: {:?}", item);
    }
}