"use strict"

export function getBit(bitOrderNumber: number, value: number): number {
    const vl = (value >> bitOrderNumber) & 1
    return vl
}

export function getBits(bitOrderNumber: number, values: number[]): number[] {
    return values.map((value => getBit(bitOrderNumber, value)))
}

export function getCommonBit(values: number[], leastCommon: boolean = false): number {
    const amountOfOneBits = values.reduce((prevValue: number, currValue: number) => { 
        return prevValue+currValue
    }, 0)
    if (leastCommon) {
        return amountOfOneBits >= values.length/2 ? 0 : 1
    }
    return amountOfOneBits >= values.length/2 ? 1 : 0
}

export function getGammaRate(values: number[], mask: number): number {
    let value = ""
    for (let n = mask.toString(2).length - 1; n >= 0; n--) {
        value = value.concat(getCommonBit(getBits(n, values)).toString())
    }
    return parseInt(value, 2)
}

export function getEpsilonRate(gammaRate: number, mask: number) {
    return gammaRate ^ mask
}

// result for part1
export function getPowerConsumption(values: number[], mask: number) {
    const gammaRate = getGammaRate(values, mask)
    const epsilonRate = getEpsilonRate(gammaRate, mask)
        console.log("gammaRate:", gammaRate)
    console.log("epsilonRate:", epsilonRate)
    return gammaRate * epsilonRate
}

export function filterValuesByCommonBits(criteriaBit: number, values: number[], leastCommon: boolean = false): number[] {
    if (criteriaBit < 0) throw Error("No Rating Found due glitch")
    const bits = getBits(criteriaBit, values)
    const commonBit = getCommonBit(bits, leastCommon)
    console.log(`most/least common bit of ${criteriaBit} is ${commonBit}`)
    let filtered = values.filter((val) => {
        return ((val >> criteriaBit) & 1) === commonBit
    })
    if (filtered.length === 1) return filtered
    if (criteriaBit < 6) console.log(filtered.map(val => val.toString(2)))
    return filterValuesByCommonBits(--criteriaBit, filtered, leastCommon)
    }

export function getOxygenGenRating(values: number[], mask: number): number {
    const lengthOfBinaries = mask.toString(2).length
    return filterValuesByCommonBits(lengthOfBinaries - 1, values)[0]
}
export function getCo2ScrubberRating(values: number[], mask: number): number {
    const lengthOfBinaries = mask.toString(2).length
    return filterValuesByCommonBits(lengthOfBinaries - 1, values, true)[0]
}
export function getLifeSupportRating(values: number[], mask: number) {
    return getOxygenGenRating(values, mask) * getCo2ScrubberRating(values, mask)
}