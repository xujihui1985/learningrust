enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePast(units, count) =>
            format!("{} {} ago", count, units.plural()),
        RoughTime::JustNow => {
            String::from("just now")
        },
        RoughTime::InTheFuture(units, count) => {
            format!("{} {} from now", count, units.plural())
        }
    }
}

enum ComplexEnum {
    Single,
    InARelationship,
    ItsComplicated(Option<String>),
    ItsExtremelyComplicated {
        car: String,
        cdr: String,
    },
}

#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

impl<T:Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match self {
            &mut BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            },
            &mut BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                }else {
                    node.right.add(value);
                }
            }
        }
    }
}

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

fn four_score_and_seven_years_ago() -> RoughTime {
    RoughTime::InThePast(TimeUnit::Years, 4 * 20 * 7)
}

impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Years => "years",
            TimeUnit::Days => "days",
            TimeUnit::Hours => "hours",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Months => "months",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_roughtime() {
        let rt = RoughTime::InTheFuture(TimeUnit::Months, 10);
        let res = rough_time_to_english(rt);
        assert_eq!("10 months from now", res.to_string());
    }

    #[test]
    fn test_binarytree() {
        let left_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
            element: "jupiter",
            left: BinaryTree::Empty,
            right: BinaryTree::Empty,
        }));
        let right_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
            element: "mercury",
            left: BinaryTree::Empty,
            right: BinaryTree::Empty,
        }));

        let root_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
            element: "root",
            left: left_tree,
            right: right_tree,
        }));
    }

    #[test]
    fn test_add_node() {
        let mut tree = BinaryTree::Empty;
        tree.add("hello");
        tree.add("world");
        tree.add("sean");
        println!("{:#?}", tree);
    }
}
