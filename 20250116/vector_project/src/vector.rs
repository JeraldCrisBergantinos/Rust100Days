// Implement a vector (mutable array with automatic resizing):
// * New raw data array with allocated memory
//     can allocate int array under the hood, just not use its features
//     start with 16, or if the starting number is greater, use power of 2 - 16, 32, 64, 128
// * size() - number of items
// * capacity() - number of items it can hold
// * is_empty()
// * at(index) - returns the item at a given index, blows up if index out of bounds
// * push(item)
// * insert(index, item) - inserts item at index, shifts that index's value and trailing elements to the right
// * prepend(item) - can use insert above at index 0
// * pop() - remove from end, return value
// * delete(index) - delete item at index, shifting all trailing elements left
// * remove(item) - looks for value and removes index holding it (even if in multiple places)
// * find(item) - looks for value and returns first index with that value, -1 if not found
// * resize(new_capacity) // private function
//     when you reach capacity, resize to double the size
//     when popping an item, if the size is 1/4 of capacity, resize to half

use std::alloc::{alloc, dealloc, Layout};

// Define a `Vector` struct with a raw pointer to data, size, and capacity
pub struct Vector {
    data: *mut i32,  // Raw pointer to a dynamically allocated array of i32
    size: usize,     // Current number of elements in the vector
    capacity: usize, // Maximum number of elements the vector can hold without resizing
}

impl Vector {
    // Creates a new `Vector` with an initial capacity, defaulting to 16 if 0 is provided
    pub fn new(initial_capacity: usize) -> Self {
        // Ensure capacity is at least 16 and is a power of two
        let capacity = if initial_capacity > 0 {
            initial_capacity.next_power_of_two()
        } else {
            16
        };

        // Allocate memory for the vector, ensuring proper layout
        let data = unsafe { alloc(Layout::array::<i32>(capacity).unwrap()) as *mut i32 };
        Vector { data, size: 0, capacity }
    }

    // Returns the current number of elements in the vector
    pub fn size(&self) -> usize {
        self.size
    }

    // Returns the current capacity of the vector
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    // Checks if the vector is empty (i.e., has no elements)
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    // Returns the element at a given index, panics if the index is out of bounds
    pub fn at(&self, index: usize) -> i32 {
        if index >= self.size {
            panic!("Index out of bounds");
        }
        // Return the value at the specified index (unsafe due to raw pointer manipulation)
        unsafe { *self.data.add(index) }
    }

    // Adds a new element to the end of the vector, resizing if necessary
    pub fn push(&mut self, item: i32) {
        // Resize if capacity is full
        if self.size == self.capacity {
            self.resize(self.capacity * 2);
        }
        // Add the item to the end and increase the size
        unsafe { *self.data.add(self.size) = item; }
        self.size += 1;
    }

    // Inserts an element at a specified index, shifting existing elements
    pub fn insert(&mut self, index: usize, item: i32) {
        if index >= self.size {
            panic!("Index out of bounds");
        }

        // Resize if capacity is full
        if self.size == self.capacity {
            self.resize(self.capacity * 2);
        }

        // Shift elements to the right starting from the specified index
        for i in (index..self.size).rev() {
            unsafe { *self.data.add(i + 1) = *self.data.add(i); }
        }

        // Insert the new item at the specified index
        unsafe { *self.data.add(index) = item; }
        self.size += 1;
    }

    // Inserts an element at the beginning of the vector
    pub fn prepend(&mut self, item: i32) {
        self.insert(0, item);
    }

    // Removes and returns the last element, resizing if necessary
    pub fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        // Get the last element
        let value = unsafe { *self.data.add(self.size - 1) };
        self.size -= 1;

        // Shrink capacity if the size is much smaller than capacity, with a minimum of 16
        if self.size <= self.capacity / 4 && self.capacity > 16 {
            self.resize(self.capacity / 2);
        }

        Some(value)
    }

    // Deletes the element at a specified index and shifts the remaining elements
    pub fn delete(&mut self, index: usize) {
        if index >= self.size {
            panic!("Index out of bounds");
        }

        // Shift elements to the left to fill the gap
        for i in index..self.size - 1 {
            unsafe { *self.data.add(i) = *self.data.add(i + 1); }
        }

        self.size -= 1;

        // Resize if necessary
        if self.size <= self.capacity / 4 && self.capacity > 16 {
            self.resize(self.capacity * 2);
        }
    }

    // Removes all occurrences of a specified item from the vector
    pub fn remove(&mut self, item: i32) {
        let mut i = 0;

        // Iterate through the vector and delete occurrences of the item
        while i < self.size {
            if unsafe { *self.data.add(i) == item } {
                self.delete(i);
            } else {
                i += 1;
            }
        }
    }

    // Finds the index of the first occurrence of an item, returns -1 if not found
    pub fn find(&self, item: i32) -> isize {
        for i in 0..self.size {
            if unsafe { *self.data.add(i) == item } {
                return i as isize;
            }
        }

        -1
    }

    // Resizes the vector's capacity and reallocates its data
    fn resize(&mut self, new_capacity: usize) {
        // Allocate new memory with the new capacity
        let new_data = unsafe { alloc(Layout::array::<i32>(new_capacity).unwrap()) as *mut i32 };

        // Copy elements from the old memory to the new memory
        for i in 0..self.size {
            unsafe { *new_data.add(i) = *self.data.add(i); }
        }

        // Deallocate the old memory
        unsafe { dealloc(self.data as *mut u8, Layout::array::<i32>(self.capacity).unwrap()) };

        self.data = new_data;
        self.capacity = new_capacity;
    }
}

// Implement the `Drop` trait to safely deallocate memory when the `Vector` is dropped
impl Drop for Vector {
    fn drop(&mut self) {
        unsafe { dealloc(self.data as *mut u8, Layout::array::<i32>(self.capacity).unwrap()) };
    }
}

// Unit tests for the `Vector` struct
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_operations() {
        let mut vec = Vector::new(0);

        // Test initial state of the vector
        assert_eq!(vec.size(), 0);
        assert_eq!(vec.capacity(), 16); // Default capacity should be 16
        assert!(vec.is_empty());

        // Test pushing elements
        vec.push(10);
        assert_eq!(vec.at(0), 10);
        assert_eq!(vec.size(), 1);

        // Test inserting elements
        vec.push(20);
        vec.insert(1, 15); // Insert 15 at index 1
        assert_eq!(vec.at(1), 15);

        // Test prepending an element
        vec.prepend(5); // Insert 5 at the beginning
        assert_eq!(vec.at(0), 5);

        // Test popping an element
        let popped_value = vec.pop().unwrap();
        assert_eq!(popped_value, 20); // Last element should be 20

        // Test deleting an element
        vec.delete(0); // Delete the first element
        assert_eq!(vec.find(15), 1);

        // Test removing an element by value
        vec.remove(15);
        assert_eq!(vec.find(15), -1); // `15` should no longer exist

        // After all operations, only one element should remain
        assert_eq!(vec.size(), 1);
    }
}