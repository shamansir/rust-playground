fn main() {

    // FIXME: make it work

    struct List {
        next: Option<~List>,
        data: int
    }

    fn prepare() -> List {
        let mut root = List { next: None, data: -1 };
        root.next = List { next: None, data: 5 };
        root.next.next = List { next: None, data: 8 };
        return root;
    }

    fn traverse(root: List) -> ~[int] {
        let mut current: Option<List> = root;
        let mut result: &[int] = [];
        while current {
            result.push(current.data);
            current = current.next;
        }
        return result;
    }

    println!("{}", traverse(prepare));

}
