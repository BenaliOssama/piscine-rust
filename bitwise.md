
### **1. Check even/odd**

```rust
if n & 1 == 0 { println!("Even"); } else { println!("Odd"); }
```

### **2. Swap two numbers without temp**

```rust
a ^= b; b ^= a; a ^= b;
```

### **3. Set, clear, toggle a bit**

```rust
// Set bit k
n |= 1 << k;
// Clear bit k
n &= !(1 << k);
// Toggle bit k
n ^= 1 << k;
```

### **4. Check power of 2**

```rust
if n != 0 && (n & (n - 1)) == 0 { println!("Power of 2"); }
```

### **5. Count 1-bits (Hamming weight)**

```rust
let mut count = 0; let mut x = n;
while x != 0 { x &= x - 1; count += 1; }
```

### **6. Get lowest set bit**

```rust
let low_bit = n & (-n);
```

### **7. Multiply/divide by 2**

```rust
let doubled = n << 1;  // multiply
let halved  = n >> 1;  // divide
```

### **8. Check if two numbers have opposite signs**

```rust
if (a ^ b) < 0 { println!("Opposite signs"); }
```

### **9. Absolute value without branching**

```rust
let abs = (n ^ (n >> 31)) - (n >> 31);
```

*(works for 32-bit integers)*

### **10. Swap even and odd bits**

```rust
let swapped = ((n & 0xAAAAAAAA) >> 1) | ((n & 0x55555555) << 1);
```

### **11. Round down to nearest power of 2**

```rust
let mut x = n;
x |= x >> 1; x |= x >> 2; x |= x >> 4;
x |= x >> 8; x |= x >> 16;
x = x - (x >> 1);
```

### **12. Isolate the most significant bit**

```rust
let msb = 1 << (31 - n.leading_zeros());
```

### **13. Check if n is multiple of 4**

```rust
if n & 3 == 0 { println!("Multiple of 4"); }
```

### **14. Bit reversal (for 8 bits)**

```rust
let reversed = ((n * 0x0202020202 & 0x010884422010) % 1023) as u8;
```

### **15. Clear lowest set bit**

```rust
n &= n - 1;
```
