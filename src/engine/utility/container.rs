pub fn count_item<'c, T>(item: &'c T, inventory: impl Iterator<Item=&'c T>) -> usize
  where T: PartialEq
{
  inventory
    .filter(|&i| i == item)
    .count()
}
