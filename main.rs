use std::collections::{hash_map, HashMap};

fn popularity_analysis(scores: Vec<i32>) -> bool {
	let mut increasing = true;
	let mut decreasing = true;

	for i in 0..scores.len() - 1 {
		if scores[i] >= scores[i+1] {
			decreasing = false;
		}
		if scores[i] <= scores[i+1] {
			increasing = false;
		}
	}

	increasing || decreasing
}

fn main() {
	let mut product_hash = HashMap::new();

	product_hash.insert("prod 1", vec![1,2,3,4]);
	product_hash.insert("prod 2", vec![4,5,6,4,7]);
	product_hash.insert("prod 3", vec![8,7,5,4,2,1]);
	
	for (id, scores) in product_hash {
		if popularity_analysis(scores) {
			println!("Popularity for {} is either increasing or decreasing", id);
		} else {
			println!("Popularity for {} is fluctuating", id);
		}
	}


}








// fn popularity_analysis(scores: Vec<i32>) -> bool {
//     let mut increasing = true;
//     let mut decreasing = true;
    
//     for i in 0..scores.len() - 1 {

//         if scores[i] > scores[i+1] {
//             increasing = false;
//         }


//         if scores[i] < scores [i+1] {
//             decreasing = false;
//         }
//     }

//     return increasing || decreasing
// }

// fn main() {
//     let mut products = HashMap::new();

// 		// inserting some products in the products hash map
//         products.insert("product 1", vec![1,2,2,3]);
// 		products.insert("product 2", vec![4,5,6,3,4]);
// 		products.insert("product 3", vec![8,8,7,6,5,4,4,1]);

// 		// we will ITERATE through all the entries in the hash map, 
// 		// by using a for loop for this purpose
// 		for (product_id, popularity) in products {
				
// 				//if the popularity_analysis() function returns true 
// 				// → we will display that the product popularity is increasing 
// 				// or decreasing using a println!(…)
// 				if popularity_analysis(popularity) {
// 						println!("{:?} popularity is increasing or decreasing", product_id);
// 				} 
				
// 				// in any other case, we will ad a statement indicating that 
// 				// the popularity is fluctuating
// 				else {
// 						println!("{:?} popularity is fluctuating", product_id);
// 				}
// 		}
// }