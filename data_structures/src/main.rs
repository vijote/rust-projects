#[derive(Debug)]
struct StackItem<T> {
    // field `value` is never read
    // `StackItem` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
    // `#[warn(dead_code)]` on by default
    _value: T,
    next: Option<Box<StackItem<T>>>,
}

impl<T> StackItem<T> {
    fn new(value: T) -> StackItem<T> {
        StackItem { _value: value, next: None }
    }
}

struct Stack<T> {
    head: Option<Box<StackItem<T>>>,
}

impl<T> Stack<T> {
    fn new(initial_value: Option<T>) -> Stack<T> {
        Stack {
            head: initial_value.map(|value| Box::new(StackItem::new(value))),
        }
    }

    fn at(&self, index: i32) -> Option<&Box<StackItem<T>>> {
        let mut current_index = 0;
        let mut current_item = self.head.as_ref();

        loop {
            if current_index == index {
                return current_item;
            }

            if current_index < index && current_item.is_some_and(|item| item.next.is_none()) {
                return None;
            }

            if current_item.is_some_and(|item| item.next.is_some()) {
                current_item = current_item.map(|item| &item.next).unwrap().as_ref()
            }

            current_index += 1;
        }
    }

    fn push(mut self, new_value: T) -> Self {
        let mut new_head = StackItem::new(new_value);

        if let Some(_) = self.head {
            new_head.next = self.head;
        }

        self.head = Some(Box::new(new_head));

        return self;
    }

    fn pop(mut self) -> Self {
        let mut new_head: Option<Box<StackItem<T>>> = None;
        
        if let Some(value) = self.head {
            if let Some(next_item) =  value.next {
                new_head = Some(next_item);
            };
        };

        self.head = new_head;

        return self;
    }
}

fn main() {
    let stack = Stack::new(Some(1));

    println!("at head {} we have: {:?}", 0, &stack.at(0));

    let stack = stack.push(2);
    println!("added to the stack!");

    println!(
        "at position {} we have: {:?}, because head moved!",
        1,
        &stack.at(1)
    );
    println!("new head: {:?}", &stack.at(0));
    println!(
        "and an invalid position that should give none: {:?}",
        &stack.at(4)
    );

    let stack = stack.pop();

    println!("removed last item! {:?}", &stack.at(0));
}
