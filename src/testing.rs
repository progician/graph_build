use std::collections::HashMap;


trait Stack {
    type Item;

    fn empty(&self) -> bool;
    fn push(&mut self, item: Self::Item);
    fn top<'a>(&self) -> Option<&'a Self::Item>;
    fn pop(&mut self) -> Option<Self::Item>;
}


struct MyStack<T> {
    vec: Vec<T>,
}

impl<T> Stack for MyStack<T> {
    type Item = T;

    fn empty(&self) -> bool {
        self.vec.is_empty()
    }

    fn push(&mut self, item: Self::Item) {
        self.vec.push(item);
    }

    fn top<'a>(&self) -> Option<&'a Self::Item> {
        None
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.vec.pop()
    }
}


impl<T> MyStack<T> {
    fn new() -> Self {
        Self {
            vec: Vec::new(),
        }
    }
}


macro_rules! require {
    ($expected:expr) => {
        if !$expected {
            return false;
        }
    };

    ($expected:expr, $actual:expr) => {
        if $expected != $actual {
            return false;
        }
    };
}


macro_rules! tests {
    ($body:block) => {
        fn main() {
            type TestCaseBody = fn() -> bool;
            struct TestCase {
                file: String,
                line: u32,
                test_case_fn: TestCaseBody,
            }
            let mut test_cases: HashMap<String, TestCase> = HashMap::new();

            macro_rules! test_case {
                ($name:expr, $case_body:block) => {
                    test_cases.insert($name.to_string(),
                        TestCase {
                            file: file!().to_string(),
                            line: line!(),
                            test_case_fn: || { $case_body; true},
                        }
                    );
                }
            }

            $body

            let mut all_succeeded = true;
            for (name, case) in test_cases {
                if !(case.test_case_fn)() {
                    println!("-------------------------------------------------------------------------------");
                    println!("{}", name);
                    println!("-------------------------------------------------------------------------------");
                    println!("{}:{}", case.file, case.line);
                    println!("...............................................................................");
                    println!("FAILED:");
        
                    all_succeeded = false;
                }
            }
        
            if !all_succeeded {
                std::process::exit(1);
            }
            else {
                std::process::exit(0);
            }
        
        }
    }
}


tests!({
    test_case!(
        "Stack is created empty",
        {
            let freshly_created_stack: MyStack<i32> = MyStack::new();
            require!(freshly_created_stack.empty());
        }
    );


    test_case!(
        "Pushing on an empty stack will no longer be empty",
        {
            let mut stack_with_items: MyStack<i32> = MyStack::new();
            stack_with_items.push(12);
            require!(!stack_with_items.empty());
        }
    );


    test_case!(
        "Popping from a stack containing a single item empties the stack",
        {
            let mut stack_with_single_item: MyStack<i32> = MyStack::new();
            const SINGLE_ITEM: i32 = 12;
            stack_with_single_item.push(SINGLE_ITEM);

            let item_popped = stack_with_single_item.pop();
            require!(stack_with_single_item.empty());
            require!(item_popped.unwrap(), SINGLE_ITEM + 1);
        }
    );
});