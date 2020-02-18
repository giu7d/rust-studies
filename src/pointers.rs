pub fn run() {
  // Primative Array
  let arr1 = [1, 2, 3];
  let arr2 = arr1;
  println!("Values: {:?}", (arr1, arr2));

  // With non-primative values, if you assign variables
  // to a piece of data, the first variable will not hold value.
  // Use & to point to the resource

  let arr1_non = vec![1, 2, 3];
  let arr2_non = &arr1_non;
  println!("Values: {:?}", (&arr1_non, arr2_non));
  println!("Values: {:?}", arr1_non);
  println!("Values: {:?}", arr2_non);
}
