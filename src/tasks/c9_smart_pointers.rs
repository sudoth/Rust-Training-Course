// This chapter is dedicated to the smart pointers: Box, Rc and RefCell.

use std::cell::RefCell;
use std::rc::Rc;

// Box
// ================================================================================================

// ----- 1 --------------------------------------
// Implement a recursive `BinaryTreeNode` which have:
// - fields:
//   - `value: i32`
//   - `left_child: Option<BinaryTreeNode>`
//   - `right_child: Option<BinaryTreeNode>`
// - methods:
//   - `new(value: i32)`, which creates a note with provided value and without any children
//   - `with_children(value: i32, left_child: BinaryTreeNode, right_child: BinaryTreeNode)` which
//     creates a note using the provided values
//   - `sum(&self)` which computes the sum of all values in the tree
//
// Use `Box` if needed

pub struct BinaryTreeNode {
    value: i32,
    left_child: Option<Box<BinaryTreeNode>>,
    right_child: Option<Box<BinaryTreeNode>>,
}

impl BinaryTreeNode {
    pub fn new(value: i32) -> Self {
        BinaryTreeNode {
            value,
            left_child: None,
            right_child: None,
        }
    }

    pub fn with_children(value: i32, left_child: BinaryTreeNode, right_child: BinaryTreeNode) -> Self {
        BinaryTreeNode {
            value,
            left_child: Some(Box::new(left_child)),
            right_child: Some(Box::new(right_child)),
        }
    }

    pub fn sum(&self) -> i32 {
        let left_sum = self.left_child.as_ref().map_or(0, |node| node.sum());
        let right_sum = self.right_child.as_ref().map_or(0, |node| node.sum());

        self.value + left_sum + right_sum
    }
}

// Rc
// ================================================================================================

// ----- 2 --------------------------------------
// Implement a package dependency tree where multiple packages can depend on the same shared
// library.
//
// Implement the `Package` struct with `name: String` and `dependencies: Vec<Package>` fields.
// Implement methods:
// - `new(name: &str) -> Self` which creates a new package with provided name and without any
//   dependencies.
// - `with_dependencies(name: &str, dependencies: Vec<Rc<Package>>) -> Self` which creates a new package
//   with provided name and dependencies.
// - `list_dependencies(package: &Rc<Package>) -> Vec<String>` which return a vector of all dependencies
//   of this package (including all recursive dependencies).
//
// Write a test which will reuse the created Packages in several other Packages as dependencies.
// Use `Rc` in the `Package` struct where needed to avoid deep clone.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub name: String,
    pub dependencies: Vec<Rc<Package>>,
}

impl Package {
    pub fn new(name: &str) -> Rc<Self> {
        Rc::new(Package {
            name: name.to_string(),
            dependencies: Vec::new(),
        })
    }

    pub fn with_dependencies(name: &str, dependencies: Vec<Rc<Package>>) -> Rc<Self> {
        Rc::new(Package {
            name: name.to_string(),
            dependencies,
        })
    }

    pub fn list_dependencies(package: &Rc<Package>) -> Vec<String> {
        let mut all_deps: Vec<String> = Vec::new();
        let mut visited: std::collections::HashSet<String> = std::collections::HashSet::new();

        fn traverse(pkg: &Rc<Package>, all_deps: &mut Vec<String>, visited: &mut std::collections::HashSet<String>) {
            for dep in &pkg.dependencies {
                if !visited.contains(&dep.name) {
                    visited.insert(dep.name.clone());
                    all_deps.push(dep.name.clone());
                    traverse(dep, all_deps, visited);
                }
            }
        }

        traverse(package, &mut all_deps, &mut visited);
        all_deps.sort();
        all_deps
    }
}


#[test]
fn test_list_dependencies() {
    let lib_a = Package::new("LibA");
    let lib_b = Package::new("LibB");
    
    let core_pkg = Package::with_dependencies(
        "Core", 
        vec![Rc::clone(&lib_a), Rc::clone(&lib_b)]
    );

    let ui_pkg = Package::with_dependencies(
        "UI", 
        vec![Rc::clone(&lib_b)]
    );

    let main_app = Package::with_dependencies(
        "MainApp",
        vec![Rc::clone(&core_pkg), Rc::clone(&ui_pkg), Package::new("Utils")]
    );

    let dependencies = Package::list_dependencies(&main_app);
    let mut expected = vec!["Core".to_string(), "UI".to_string(), "Utils".to_string(), "LibA".to_string(), "LibB".to_string()];
    expected.sort();

    assert_eq!(dependencies.len(), 5);

    let mut expected_deps = vec![
        "Core".to_string(), 
        "UI".to_string(), 
        "Utils".to_string(), 
        "LibA".to_string(), 
        "LibB".to_string()
    ];
    expected_deps.sort();

    assert_eq!(dependencies, expected_deps);
    
    assert_eq!(Rc::strong_count(&lib_b), 3);
    assert_eq!(Rc::strong_count(&lib_a), 2);
}

// RefCell
// ================================================================================================

// ----- 3 --------------------------------------
// Create a simple `SharedCounter` where multiple owners can increment its value without mutable
// reference.
//
// Implement `new() -> Self` constructor, `increment(&self)` and `get(&self) -> i32` methods.
// Use `RefCell` where needed.

pub struct SharedCounter {
    value: RefCell<i32>,
}

impl SharedCounter {
    pub fn new() -> Self {
        SharedCounter {
            value: RefCell::new(0),
        }
    }

    pub fn increment(&self) {
        *self.value.borrow_mut() += 1;
    }

    pub fn get(&self) -> i32 {
        *self.value.borrow()
    }
}