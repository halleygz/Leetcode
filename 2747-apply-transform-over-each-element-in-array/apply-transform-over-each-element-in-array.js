/**
 * @param {number[]} arr
 * @param {Function} fn
 * @return {number[]}
 */
var map = function(arr, fn) {
    let newArr = arr.map(fn)
    return newArr
};

const fun = function plusone(n) {return n+1}
map([1,2,3], fun)
