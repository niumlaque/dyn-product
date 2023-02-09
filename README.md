# DYN-PRODUCT
A crate that creates cartesian product of size determined at runtime.  
If the size is determined at compile time, it is better to use `itertools::iproduct!`.

# Usage
Add dependency to your toml.
```toml
[dependencies]
dyn-product = { git = "https://github.com/niumlaque/dyn-product", branch = "main" }
```

# Example
```no_run
use dyn_product::DynProduct;

let data = vec![
    vec!["GroupA-1", "GroupA-2", "GroupA-3"],
    vec!["GroupB-1", "GroupB-2"],
    vec!["GroupC-1", "GroupC-2", "GroupC-3", "GroupC-4"],
];

for item in DynProduct::from(&data) {
    println!("{:?}", item);
}
```
output:
```text
["GroupA-1", "GroupB-1", "GroupC-1"]
["GroupA-1", "GroupB-1", "GroupC-2"]
["GroupA-1", "GroupB-1", "GroupC-3"]
["GroupA-1", "GroupB-1", "GroupC-4"]
["GroupA-1", "GroupB-2", "GroupC-1"]
["GroupA-1", "GroupB-2", "GroupC-2"]
["GroupA-1", "GroupB-2", "GroupC-3"]
["GroupA-1", "GroupB-2", "GroupC-4"]
["GroupA-2", "GroupB-1", "GroupC-1"]
["GroupA-2", "GroupB-1", "GroupC-2"]
["GroupA-2", "GroupB-1", "GroupC-3"]
["GroupA-2", "GroupB-1", "GroupC-4"]
["GroupA-2", "GroupB-2", "GroupC-1"]
["GroupA-2", "GroupB-2", "GroupC-2"]
["GroupA-2", "GroupB-2", "GroupC-3"]
["GroupA-2", "GroupB-2", "GroupC-4"]
["GroupA-3", "GroupB-1", "GroupC-1"]
["GroupA-3", "GroupB-1", "GroupC-2"]
["GroupA-3", "GroupB-1", "GroupC-3"]
["GroupA-3", "GroupB-1", "GroupC-4"]
["GroupA-3", "GroupB-2", "GroupC-1"]
["GroupA-3", "GroupB-2", "GroupC-2"]
["GroupA-3", "GroupB-2", "GroupC-3"]
["GroupA-3", "GroupB-2", "GroupC-4"]
```