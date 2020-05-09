trait Slicer<'a> {
    type Output: 'a;

    fn limit(&self, pred: impl Fn(i32) -> bool) -> Self::Output;
}

impl<'a> Slicer<'a> for &'a [i32] {
    type Output = &'a [i32];

    fn limit(&self, pred: impl Fn(i32) -> bool) -> &'a [i32] {
        let (idx, _) = self
            .iter()
            .enumerate()
            .find(|(_, item)| pred(**item))
            .unwrap_or((self.len(), &0));

        &self[idx..]
    }
}

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let slice: &[i32] = &data;
    println!("Slice: {:?}", slice);
    let slice = slice.limit(|x| x > 3);
    println!("Slice: {:?}", slice);
    let slice = slice.limit(|x| x % 2 == 1);
    println!("Slice: {:?}", slice);
}
