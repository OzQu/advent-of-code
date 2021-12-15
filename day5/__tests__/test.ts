"use strict"

import { test, expect } from '@jest/globals'

import { OceanFloor } from '../lib/OceanFloor'

// test('corner cases', async () => {
//     const limit: number = 2
//     const tooDangerousZones: number = 5
//     const oceanFloor: OceanFloor = new OceanFloor("./__tests__/cornerCases.txt", true, true)
//     const count = await oceanFloor.count(limit)
//     expect(count).toEqual(tooDangerousZones)
// })

test('horizontal and vertical example data', async () => {
    const limit: number = 2
    const tooDangerousZones: number = 5
    const oceanFloor: OceanFloor = new OceanFloor("./__tests__/testInput.txt", true, false)
    const count = await oceanFloor.count(limit)
    expect(count).toEqual(tooDangerousZones)
})

test('45 diagonals example data', async () => {
    const limit: number = 2
    const tooDangerousZones: number = 12
    const oceanFloor: OceanFloor = new OceanFloor("./__tests__/testInput.txt", true, true)
    const count = await oceanFloor.count(limit)
    expect(count).toEqual(tooDangerousZones)
})