extern crate simple_kbuckets;

use simple_kbuckets::Table;

#[test]
fn retrieve_one_exact() {
    let mut table = Table::new(0b0101, 4, 3);
    table.update(0b1111, 42);
    table.update(0b0000, 43);
    table.update(0b1010, 44);
    table.update(0b0101, 45);
    assert_eq!(table.find(&0b1111, 1), vec![(0b1111, &42)]);
    assert_eq!(table.find(&0b0000, 1), vec![(0b0000, &43)]);
    assert_eq!(table.find(&0b1010, 1), vec![(0b1010, &44)]);
    assert_eq!(table.find(&0b0101, 1), vec![(0b0101, &45)]);
}

#[test]
fn retrieve_one_closest() {
    let mut table = Table::new(0b0101, 4, 3);
    table.update(0b1111, 42);
    table.update(0b0000, 43);
    table.update(0b1010, 44);
    table.update(0b0101, 45);
    assert_eq!(table.find(&0b1110, 1), vec![(0b1111, &42)]);
    assert_eq!(table.find(&0b0001, 1), vec![(0b0000, &43)]);
    assert_eq!(table.find(&0b1000, 1), vec![(0b1010, &44)]);
    assert_eq!(table.find(&0b0111, 1), vec![(0b0101, &45)]);
}

#[test]
fn bucket_ageing() {
    let mut table = Table::new(0b0101, 4, 3);
    table.update(0b1101, 42);
    table.update(0b1111, 43);
    table.update(0b1100, 44);
    assert_eq!(table.find(&0b0101, 4), vec![(0b1101, &42), (0b1100, &44), (0b1111, &43)]);

    table.update(0b1110, 45);
    assert_eq!(table.find(&0b0101, 4), vec![(0b1100, &44), (0b1111, &43), (0b1110, &45)]);
}

#[test]
fn bucket_ageing_refresh() {
    let mut table = Table::new(0b0101, 4, 3);
    table.update(0b1101, 42);
    table.update(0b1111, 43);
    table.update(0b1100, 44);
    assert_eq!(table.find(&0b0101, 4), vec![(0b1101, &42), (0b1100, &44), (0b1111, &43)]);

    table.update(0b1101, 100);
    assert_eq!(table.find(&0b0101, 4), vec![(0b1101, &100), (0b1100, &44), (0b1111, &43)]);

    table.update(0b1110, 45);
    assert_eq!(table.find(&0b0101, 4), vec![(0b1101, &100), (0b1100, &44), (0b1110, &45)]);
}
