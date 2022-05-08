"use strict"

import { getBit, getCo2ScrubberRating, getLifeSupportRating, getOxygenGenRating } from '../lib'
import { test, expect } from '@jest/globals'

const exampleInput = [
    0b00100,
    0b11110,
    0b10110,
    0b10111,
    0b10101,
    0b01111,
    0b00111,
    0b11100,
    0b10000,
    0b11001,
    0b00010,
    0b01010,
]

test('bitshifting', () => {
    const four = 0b0000000100
    const secondBit = getBit(3, four)
    expect(secondBit).toEqual(0)
    const thirdBit = getBit(2, four)
    expect(thirdBit).toEqual(1)
    const fourthBit = getBit(3, four)
    expect(fourthBit).toEqual(0)
})

test('example data', () => {
    expect(getOxygenGenRating(exampleInput, 0x1f)).toEqual(23)
    expect(getCo2ScrubberRating(exampleInput, 0x1f)).toEqual(10)
    expect(getLifeSupportRating(exampleInput, 0x1f)).toEqual(230)
})