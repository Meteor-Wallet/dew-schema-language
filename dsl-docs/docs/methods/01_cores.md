---
title: Core DSL Functions
---

# Core DSL Functions

---

## 🔗 `and(...args)`

**Description:**  
Performs a logical **AND** between the current value (callee) and one or more boolean arguments.

**Parameters:**

-   `args` – One or more `Boolean`.

**Returns:**  
`Boolean`

**Errors:**

-   Throws if any argument is not a boolean.
-   Throws if no arguments are provided.

**Example:**

```rust
true.and(true, false) // => false
and(true, false) // => false
```

---

## 🔗 `or(...args)`

**Description:**  
Performs a logical **OR** between the current value (callee) and one or more boolean arguments.

**Parameters:**

-   `args` – One or more `Boolean`.

**Returns:**  
`Boolean`

**Errors:**

-   Throws if any argument is not a boolean.
-   Throws if no arguments are provided.

**Example:**

```rust
false.or(true, false) // => true
or(true, false) // => true
```

---

## 🔗 `equal(arg)`

**Description:**  
Checks if the callee is equal to the given argument.

**Parameters:**

-   `arg` – Any `DewSchemaLanguageResult`.

**Returns:**  
`Boolean`

**Errors:**

-   Throws if more than one argument is provided.
-   Throws if the callee is `null`.

**Example:**

```rust
5.equal(5) // => true
```

---

## 🔗 `gte(arg)`

**Description:**  
Checks if the callee is greater than or equal to the given argument.

**Parameters:**

-   `arg` – Must be a `Number`.

**Returns:**  
`Boolean`

**Errors:**

-   Throws if not exactly one argument is provided.
-   Throws if callee or arg is not numeric.

**Example:**

```rust
10.gte(5) // => true
```

---

## 🔗 `lte(arg)`

**Description:**  
Checks if the callee is less than or equal to the given argument.

**Parameters:**

-   `arg` – Must be a `Number`.

**Returns:**  
`Boolean`

**Errors:**

-   Throws if not exactly one argument is provided.
-   Throws if callee or arg is not numeric.

**Example:**

```rust
3.lte(5) // => true
```

---

## Summary

| Function       | Description                 | Returns   |
| -------------- | --------------------------- | --------- |
| `and(...args)` | Logical AND across booleans | `Boolean` |
| `or(...args)`  | Logical OR across booleans  | `Boolean` |
| `equal(arg)`   | Equality check              | `Boolean` |
| `gte(arg)`     | Greater than or equal to    | `Boolean` |
| `lte(arg)`     | Less than or equal to       | `Boolean` |

---

💡 Tip: These functions are chainable since they all return a `DewSchemaLanguageResult`.
