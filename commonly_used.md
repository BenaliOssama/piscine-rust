### **1. Iterator Tricks**

* `peekable()` – look ahead in an iterator
* `enumerate()` – get index + value
* `rev()` – reverse iteration
* `zip()` – combine two iterators
* `chain()` – concatenate iterators
* `flat_map()` – map and flatten

---

### **2. Option/Result Advanced**

* `ok_or("error")?` – turn Option into Result quickly
* `unwrap_or_else(|| expensive_calc())` – lazy default
* `get_or_insert(value)` – insert in Option if None
* `get_or_insert_with(|| compute())` – lazy insert

---

### **3. Slices & Collections**

* `split_at(index)` – split slice in two
* `chunks(n)` / `windows(n)` – iterate over subarrays
* `retain(|x| cond)` – filter in-place
* `drain(range)` – remove and return elements

---

### **4. String & Str Tricks**

* `trim()` – remove whitespace
* `lines()` – iterate lines
* `chars()` – iterate characters
* `split_once("sep")` – split at first occurrence

---

### **5. Smart Borrowing**

* `Cow` – “clone-on-write” for efficient strings/arrays
* `as_ref()` / `as_mut()` – convert \&Option<T> to Option<\&T>
* `to_owned()` – clone cheaply when needed

---

### **6. Functional Patterns**

* `fold(init, |acc, x| ...)` – reduce to a value
* `any()` / `all()` – boolean checks on iterator
* `find_map(|x| Some(...))` – combine map + find

---

### **7. Rust Macros That Save Time**

* `dbg!()` – quick debug with file/line
* `format!()` – build strings
* `vec![val; n]` – repeat values
* `concat!()` – compile-time string concat

---

### **8. Error Handling Shortcuts**

* `?` everywhere instead of nested match
* `map_err(|e| e.into())?` – convert error types quickly
* `Option.ok_or_else(|| MyError)?` – Option → Result elegantly

---

### **9. Misc Tricks**

* `..` in patterns to ignore parts
* `matches!(expr, pattern)` – bool check for pattern
* `std::mem::replace(&mut x, new)` – swap values safely
* `std::mem::take(&mut x)` – take ownership leaving default
