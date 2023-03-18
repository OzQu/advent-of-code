"use strict"

import { Lanternfish } from './lib/Lanternfish'

(async() => {
    try {
        const inputFile: string = "./day6InputData.txt"
        console.log(`inputFile: ${inputFile}`)

        const lanternfish = new Lanternfish(inputFile)
        console.log(lanternfish)
    } catch (error) {
        console.log(error)
    }
})()
