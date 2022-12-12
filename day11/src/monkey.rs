use crate::item::ItemWorry;
use crate::monkey_operation::MonkeyOperation;

use std::collections::VecDeque;

pub struct Monkey {
    id: usize,
    items: VecDeque<ItemWorry>,
    operation: MonkeyOperation,
    test_value: i32,
    items_processed: i32,
    true_monkey_id: usize,
    false_monkey_id: usize
}

impl Monkey {
    pub fn new(id: usize,
            items: VecDeque<ItemWorry>, 
            operation: MonkeyOperation, 
            test_value: i32,
            true_id: usize,
            false_id: usize)
        -> Monkey {

        Monkey {
            id: id,
            items: items,
            operation: operation,
            test_value: test_value,
            items_processed: 0,
            true_monkey_id: true_id,
            false_monkey_id: false_id
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn receive_item(&mut self, item: ItemWorry) {
        self.items.push_back(item);
    }
    
    pub fn get_items_processed(&self) -> i32 {
        self.items_processed
    }

    pub fn get_throw_monkey_id(&self, test_result: bool) -> usize {
        if test_result {
            self.true_monkey_id
        } else {
            self.false_monkey_id
        }
    }

    pub fn has_items(&self) -> bool {
        self.items.len() > 0
    }

    pub fn handle_next_item(&mut self) -> Option<(ItemWorry, bool)> {
        match self.items.pop_front(){
            // items left to process - handle next item available
            Some(item) => {
                self.items_processed += 1;

                let mut next_item = item;
                println!(" > Monkey {} inspects an item, worry level {}.", self.id, next_item);

                next_item = self.operation.apply(next_item);
                println!("  > Worry level increases to {}", next_item);

                next_item = next_item / 3;
                println!("  > Monkey gets bored. Worry level level divides to {}", next_item);

                let item_test_result = next_item % self.test_value == 0;
                println!("  > Monkey tests {} % {}? {}.\n  > Monkey throws it to monkey {}", next_item, self.test_value, item_test_result, self.get_throw_monkey_id(item_test_result));

                next_item = next_item % 3;

                Some((next_item, item_test_result))
            }

            // no items available - return Option::None
            None => None
        }

    }

}

impl std::fmt::Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Monkey {}:\n\titems: {:#?}\n\toperation: {}\n\ttest: item % {}\n\ttrue id: {}\n\tfalse id {}", 
            self.id,
            self.items,
            self.operation,
            self.test_value,
            self.true_monkey_id,
            self.false_monkey_id)
    }
}