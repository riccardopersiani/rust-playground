fn main() {
   let mut v: Vec<i32> = Vec::new();
   v = vec![1, 2, 3];
   let mut v1 = Vec::new();
   v1.push(5);
   v1.push(6);
   v1.push(7);
   v1.push(8);

   let third: &i32 = &v[2];
   let third: Option<&i32> = v.get(2);
}
