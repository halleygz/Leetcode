/**
 * @param {number} n
 * @return {Function} counter
 */
var createCounter = function(n) {
    let count = n-1
    return function() {
        count += 1
        return count
    };
};


const counter = createCounter(-2)
counter() // 10
counter() // 11
counter() // 12
