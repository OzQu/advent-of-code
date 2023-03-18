"use strict"

import { Game } from './lib/Game'

(async() => {
    try {
        const inputFile: string = "./day4InputData.txt"

        console.log(`inputFile: ${inputFile}`)
        const game: Game = new Game(inputFile)
        const winnerScore = await game.play()
        console.log(`Winner score ${winnerScore}`)

        const game2: Game = new Game(inputFile)
        const winnerScore2 = await game2.play(true)
        console.log(`Winner score ${winnerScore2}`)
    } catch (error) {
        console.log(error)

    }
})()