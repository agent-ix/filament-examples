use core::ops::ControlFlow;
use filament_walk::{walk, Arena, Children, Id, Walk};

struct Tree(Vec<Vec<usize>>);

struct Count<'a> {
    tree: &'a Tree,
    entered: usize,
    stop_at: Option<usize>,
}

impl Walk for Count<'_> {
    type Node = usize;
    type Frame = usize;
    type Stop = &'static str;

    fn enter(
        &mut self,
        node: usize,
        children: &mut Children<'_, usize>,
    ) -> ControlFlow<&'static str, usize> {
        if self.stop_at == Some(node) {
            return ControlFlow::Break("found");
        }
        self.entered += 1;
        children.extend(self.tree.0[node].iter().copied());
        ControlFlow::Continue(node)
    }

    fn exit(&mut self, _frame: usize) -> ControlFlow<&'static str> {
        ControlFlow::Continue(())
    }
}

// A chain 100,000 nodes deep: node i has the single child i + 1.
fn chain(depth: usize) -> Tree {
    Tree((0..depth).map(|i| if i + 1 < depth { vec![i + 1] } else { vec![] }).collect())
}

fn main() {
    let tree = Tree(vec![vec![1, 2], vec![], vec![3], vec![]]);
    let mut count = Count { tree: &tree, entered: 0, stop_at: None };
    let end = walk(&mut count, 0);
    println!("small tree: {:?}, entered {}", end, count.entered);

    let mut find = Count { tree: &tree, entered: 0, stop_at: Some(2) };
    println!("stop at node 2: {:?}, entered {}", walk(&mut find, 0), find.entered);

    // The walk runs on a thread whose native stack is 512 KiB.
    let depth = 100_000;
    let out = std::thread::Builder::new()
        .stack_size(512 * 1024)
        .spawn(move || {
            let deep = chain(depth);
            let mut count = Count { tree: &deep, entered: 0, stop_at: None };
            let end = walk(&mut count, 0);
            (end, count.entered)
        })
        .unwrap()
        .join()
        .unwrap();
    println!("chain of {depth}: {:?}, entered {}", out.0, out.1);

    // Bottom-up over an arena: each node is stored after its children.
    enum Expr {
        Leaf(i64),
        Add(Id<Expr>, Id<Expr>),
    }
    let mut arena = Arena::new();
    let one = arena.push(Expr::Leaf(1));
    let two = arena.push(Expr::Leaf(2));
    let sum = arena.push(Expr::Add(one, two));
    let top = arena.push(Expr::Add(sum, two));
    let values = arena.bottom_up(|node, done| match node {
        Expr::Leaf(n) => *n,
        Expr::Add(a, b) => done.get(*a).unwrap() + done.get(*b).unwrap(),
    });
    println!("arena of {} nodes, value of the top node: {}", arena.len(), values.get(top).unwrap());
}
