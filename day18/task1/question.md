Hello i'm solving AOC in rust and got stuck. I have following self referential types:

```rust
#[derive(Debug, PartialEq, Clone)]
pub enum SnailNumberValue {
    Regular(u64),
    Complex {
        left: Box<SnailNumberValue>,
        right: Box<SnailNumberValue>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct SnailFishNumber {
    left: SnailNumberValue,
    right: SnailNumberValue,
}
```

According to task I would have to locate elements with depth equal to 4 and
split replace it with some value while incrementing nearby elements.
I cannot wrap my head around the mutability here. 
Is there a way that i could find reference to interesting
SnailNumberValue    
