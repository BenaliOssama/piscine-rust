
## **1. Iterators**

| Method               | Use                        |              |                        |
| -------------------- | -------------------------- | ------------ | ---------------------- |
| `iter().enumerate()` | index + value              |              |                        |
| `iter().rev()`       | reverse iteration          |              |                        |
| `iter().peekable()`  | look ahead                 |              |                        |
| `zip()`              | combine two iterators      |              |                        |
| `chain()`            | concatenate iterators      |              |                        |
| `flat_map()`         | map + flatten              |              |                        |
| \`find\_map(         | x                          | Some(...))\` | combine map + find     |
| \`fold(init,         | acc, x                     | ...)\`       | reduce to single value |
| `any()` / `all()`    | boolean checks on iterator |              |                        |

---

## **2. Option / Result**

| Method / Pattern         | Use                     |                      |                         |
| ------------------------ | ----------------------- | -------------------- | ----------------------- |
| `ok_or("err")?`          | convert Option → Result |                      |                         |
| \`unwrap\_or\_else(      |                         | expensive\_calc())\` | lazy default            |
| `get_or_insert(value)`   | insert if None          |                      |                         |
| \`get\_or\_insert\_with( |                         | compute())\`         | lazy insert             |
| \`map\_err(              | e                       | e.into())?\`         | convert error types     |
| \`Option.ok\_or\_else(   |                         | MyError)?\`          | elegant Option → Result |

---

## **3. Slices & Collections**

| Method                     | Use                               |         |                 |
| -------------------------- | --------------------------------- | ------- | --------------- |
| `split_at(index)`          | split slice                       |         |                 |
| `chunks(n)` / `windows(n)` | iterate subarrays                 |         |                 |
| \`retain(                  | x                                 | cond)\` | in-place filter |
| `drain(range)`             | remove & return elements          |         |                 |
| `to_owned()`               | cheap clone of string/array       |         |                 |
| `as_ref()` / `as_mut()`    | convert \&Option<T> → Option<\&T> |         |                 |

---

## **4. Strings & Str**

| Method                  | Use                       |
| ----------------------- | ------------------------- |
| `trim()`                | remove whitespace         |
| `lines()`               | iterate lines             |
| `chars()`               | iterate characters        |
| `split_once("sep")`     | split at first occurrence |
| `push_str()` / `push()` | append string or char     |

---

## **5. Macros**

| Macro          | Use                        |
| -------------- | -------------------------- |
| `dbg!(var)`    | debug with file/line       |
| `format!()`    | build strings              |
| `vec![val; n]` | repeat values              |
| `concat!()`    | compile-time string concat |

---

## **6. Borrowing & Memory**

| Pattern / Function               | Use                           |
| -------------------------------- | ----------------------------- |
| `&vec`                           | borrow, don’t move            |
| `Cow`                            | clone-on-write (efficient)    |
| `std::mem::replace(&mut x, new)` | swap values safely            |
| `std::mem::take(&mut x)`         | take ownership, leave default |

---

## **7. Pattern Matching / Shortcuts**

| Pattern / Trick              | Use                                   |
| ---------------------------- | ------------------------------------- |
| `if let Some(x) = option {}` | simpler match                         |
| `..`                         | ignore parts in struct/tuple patterns |
| `matches!(expr, pattern)`    | bool check for pattern                |

---

### **Extra Practical Notes**

* Prefer **iterators** over for-loops for concise, functional code.
* Use **`?`** everywhere for error propagation.
* Use **`retain()`** or **`drain()`** for in-place collection modification.
* Use **`flat_map` + `filter`** to avoid nested loops.
* **`Cow`** saves memory when you sometimes need owned and sometimes borrowed data.
