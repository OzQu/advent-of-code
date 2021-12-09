"use strict"

import { getLifeSupportRating, getPowerConsumption } from "./lib"
import { values } from "./day3InputValues"
const bitMask12: number = 0xfff

console.log("Values:", values)
console.log("power consumption:", getPowerConsumption(values, bitMask12))
console.log(`Life Support Rating: ${getLifeSupportRating(values, bitMask12)}`)


