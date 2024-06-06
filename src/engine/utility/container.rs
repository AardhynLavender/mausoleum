/**
 * Generic container utility functions
 */

pub fn count_if<'c, T: 'c>(inventory: impl Iterator<Item=&'c T>, predicate: impl Fn(&T) -> bool) -> usize {
  inventory
    .filter(|&i| predicate(i))
    .count()
}

pub fn count<'c, T>(item: &'c T, inventory: impl Iterator<Item=&'c T>) -> usize
  where T: PartialEq
{
  count_if(inventory, |i| i == item)
}
