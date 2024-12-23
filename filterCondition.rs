// FilterCondition struct definition
struct FilterCondition {
    value: i32, // Field for filtering condition
}

// Implement is_match method for FilterCondition
impl FilterCondition {
    fn is_match(&self, item: &i32) -> bool {
        *item == self.value
    }
}

// Define custom_filter function
fn custom_filter(collection: Vec<i32>, filter: &FilterCondition) -> Vec<i32> {
    let mut filtered_collection = Vec::new();

    for item in collection {
        if filter.is_match(&item) {
            filtered_collection.push(item);
        }
    }

    filtered_collection
}

// Main function
fn main() {
    let collection = vec![1, 2, 3, 4, 5, 6]; // Example collection
    let filter_condition = FilterCondition { value: 3 }; // Example filter condition

    let filtered_collection = custom_filter(collection, &filter_condition);

    println!("Filtered Collection: {:?}", filtered_collection);
}
