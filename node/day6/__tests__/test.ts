"use strict"

import { test, expect } from '@jest/globals'
import { Lanternfish } from '../lib/Lanternfish'

test('example data', async () => {
    const total: number = 5934
    const days: number = 80
    const oceanFloor: Lanternfish = new Lanternfish("./__tests__/cornerCases.txt")
    Lanternfish.count(days)
    expect(count).toEqual(tooDangerousZones)
})