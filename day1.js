"use strict"

const inputValues = require('./day1InputValues')
// console.log(inputValues)

function parseInts(array) {
    return array.map((x) => parseInt(x))
}

function countIncreasedSteps(array) {
    let increased = 0
    array.reduce((a,b) => {
        if (a === undefined || b === undefined) return b
        if (b > a) increased++ // side effect!
        return b
    })
    return increased
}

console.log(countIncreasedSteps(parseInts(inputValues)))

function slidingAverage(amount, array) {
    let averages = []
    array.map((value, index, array) => {
        if (index < amount - 1) {
            averages.push(undefined)
            return value
        } 
        const end = index + 1
        const start = end - amount > 0 ? end - amount : 0
        averages.push(array.slice(start, end).reduce((a,b) => a+b))
        return value
    })
    return averages
}

console.log(countIncreasedSteps(slidingAverage(3, parseInts(inputValues))))
