#[derive(Debug)]
struct Slicer<'a> {
    part: &'a [i32],
}

impl<'a> Slicer<'a> {
    fn limit(&self, pred: impl Fn(i32) -> bool) -> Slicer<'a> {
        let (idx, _) = self.part
            .iter()
            .enumerate()
            .find(|(_, item)| pred(**item))
            .unwrap_or((self.part.len(), &0));

        Slicer {
            part: &self.part[idx..],
        }
    }
}

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let slicer = Slicer { part: &data };
    println!("Slicer: {:?}", slicer);
    let slicer = slicer.limit(|x| x > 3);
    println!("Slicer: {:?}", slicer);
    let slicer = slicer.limit(|x| x % 2 == 1);
    println!("Slicer: {:?}", slicer);
}
