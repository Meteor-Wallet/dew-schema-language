---
title: Alias Functions
---

# Alias Functions

---

## 🔗 `assert_equal(arg)`

**Description:**  
Asserts that the callee is equal to the given argument.  
Internally calls [`equal`](./dsl-functions.md#-equalarg).

**Parameters:**

-   `arg` – Any `DewSchemaLanguageResult`.

**Returns:**  
`Boolean(true)` if the assertion passes.

**Errors:**

-   Throws if the callee is not equal to `arg`.
-   Error message:

```

<callee> is not equal to <arg>

```

**Example:**

```rust
5.assert_equal(5) // => true
5.assert_equal(7) // => Error: 5 is not equal to 7
```

---

## 🔗 `assert_gte(arg)`

**Description:**  
Asserts that the callee is greater than or equal to the given argument.  
Internally calls [`gte`](./dsl-functions.md#-gtearg).

**Parameters:**

-   `arg` – Must be a `Number`.

**Returns:**  
`Boolean(true)` if the assertion passes.

**Errors:**

-   Throws if the callee is less than `arg`.
-   Error message:
    ```
    <callee> is not greater than or equal to <arg>
    ```

**Example:**

```rust
10.assert_gte(5) // => true
3.assert_gte(5)  // => Error: 3 is not greater than or equal to 5
```

---

## 🔗 `assert_lte(arg)`

**Description:**  
Asserts that the callee is less than or equal to the given argument.  
Internally calls [`lte`](./dsl-functions.md#-ltearg).

**Parameters:**

-   `arg` – Must be a `Number`.

**Returns:**  
`Boolean(true)` if the assertion passes.

**Errors:**

-   Throws if the callee is greater than `arg`.
-   Error message:
    ```
    <callee> is not less than or equal to <arg>
    ```

**Example:**

```rust
3.assert_lte(5)  // => true
10.assert_lte(5) // => Error: 10 is not less than or equal to 5
```

---

## Summary

| Function       | Description                         | Returns                  |
| -------------- | ----------------------------------- | ------------------------ |
| `assert_equal` | Asserts equality check              | `Boolean(true)` or Error |
| `assert_gte`   | Asserts greater-than-or-equal check | `Boolean(true)` or Error |
| `assert_lte`   | Asserts less-than-or-equal check    | `Boolean(true)` or Error |

---
