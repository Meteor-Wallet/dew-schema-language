export declare interface DslResult {
    /**
     * Numeric addition.
     *
     * Example:
     * ```js
     * 5.add(3) // 8
     * ```
     */
    add(arg: DslResult): DslResult;

    /**
     * Logical AND — combines multiple boolean expressions.
     *
     * Example:
     * ```js
     * 5.gte(3).and(true) // true
     * ```
     */
    and(...args: Array<DslResult>): DslResult;

    /**
     * Normal .equal() but throws an error if not equal.
     *
     * Example:
     * ```js
     * 5.assert_equal(5) // true
     * ```
     */
    assert_equal(arg: DslResult): DslResult;

    /**
     * Normal .gte() but throws an error if not greater than or equal.
     *
     * Example:
     * ```js
     * 10.assert_gte(5) // true
     * ```
     */
    assert_gte(arg: DslResult): DslResult;

    /**
     * Normal .lte() but throws an error if not less than or equal.
     *
     * Example:
     * ```js
     * 2.assert_lte(5) // true
     * ```
     */
    assert_lte(arg: DslResult): DslResult;

    /**
     * Numeric division.
     *
     * Example:
     * ```js
     * 10.divide(2) // 5
     * ```
     */
    divide(arg: DslResult): DslResult;

    /**
     * Equality check.
     *
     * Example:
     * ```js
     * "hello".equal("hello") // true
     * ```
     */
    equal(arg: DslResult): DslResult;

    /**
     * Greater than or equal comparison.
     *
     * Example:
     * ```js
     * 10.gte(5) // true
     * ```
     */
    gte(arg: DslResult): DslResult;

    /**
     * Inclusion check — returns true if the value is found within an array or string.
     *
     * Example:
     * ```js
     * 2.in([1, 2, 3]) // true
     * ```
     */
    in(arg: DslResult): DslResult;

    /**
     * Returns the length of a string or array.
     *
     * Example:
     * ```js
     * "hello".length() // 5
     * [1, 2, 3].length() // 3
     * ```
     */
    length(): DslResult;

    /**
     * Less than or equal comparison.
     *
     * Example:
     * ```js
     * 2.lte(5) // true
     * ```
     */
    lte(arg: DslResult): DslResult;

    /**
     * Numeric multiplication.
     *
     * Example:
     * ```js
     * 5.multiply(3) // 15
     * ```
     */
    multiply(arg: DslResult): DslResult;

    /**
     * Logical OR — returns true if at least one expression is true.
     *
     * Example:
     * ```js
     * false.or(true) // true
     * ```
     */
    or(...args: Array<DslResult>): DslResult;

    /**
     * Percentage calculation — calculates the given percentage of the number.
     *
     * Example:
     * ```js
     * 100.percent(15) // 15
     * ```
     */
    percent(arg: DslResult): DslResult;

    /**
     * Numeric subtraction.
     *
     * Example:
     * ```js
     * 5.subtract(3) // 2
     * ```
     */
    subtract(arg: DslResult): DslResult;

    /**
     * Converts the value to a number.
     *
     * Example:
     * ```js
     * "123".to_number() // 123
     * ```
     */
    to_number(): DslNumber;

    [key: string]: DslResult;
}

declare global {
    /**
     * Root object.
     *
     * Example:
     * ```js
     * $.propertyName // Access a property on the root object
     * ```
     */
    const $: DslResult;

    /**
     * Global function: logical AND.
     * Equivalent to using .and() on a DslResult.
     *
     * Example:
     * ```js
     * and(1.gte(0), true) // true
     * ```
     */
    declare function and(...args: Array<DslResult>): DslResult;

    /**
     * Global function: creates an array of DslResult items.
     *
     * Example:
     * ```js
     * array(1, "two", true) // [1, "two", true]
     * ```
     */
    declare function array(...args: Array<DslResult>): DslResult;

    /**
     * Global function: logical OR.
     * Equivalent to using .or() on a DslResult.
     *
     * Example:
     * ```js
     * or(false, true) // true
     * ```
     */
    declare function or(...args: Array<DslResult>): DslResult;
}
