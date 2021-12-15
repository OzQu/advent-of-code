"use strict"

import { OceanFloor } from './lib/OceanFloor'

(async() => {
    try {
        const inputFile: string = "./day5InputData.txt"
        console.log(`inputFile: ${inputFile}`)

        const oceanFloor = new OceanFloor(inputFile, true)

        const limit = 2
        console.log(await oceanFloor.count(limit))
        
        const oceanFloor2 = new OceanFloor(inputFile, true, true)

        console.log(await oceanFloor2.count(limit))

    } catch (error) {
        console.log(error)
    }
})()
