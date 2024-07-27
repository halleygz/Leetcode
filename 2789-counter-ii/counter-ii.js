/**
 * @param {integer} init
 * @return { increment: Function, decrement: Function, reset: Function }
 */
var createCounter = function(init) {
    let counter = init
    return {
        increment(){
            return counter+=1
        },
        reset(){
            return counter=init
        },
        decrement(){
            return counter-=1
        }
    }
};

const counter = createCounter(5)
counter.increment(); // 6
/**
 * counter.decrement(); // 4
 * counter.reset(); // 5
 */