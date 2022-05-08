"use strict"

import { test, expect } from '@jest/globals'

import { Game } from '../lib/Game'

test('example data', async () => {
    const exampleFinalScore : number = 4512
    const game: Game = new Game("./__tests__/testInput.txt")
    const winnerScore = await game.play()
    expect(winnerScore).toEqual(exampleFinalScore)
})

test('example data last winner', async () => {
    const exampleFinalScore : number = 1924
    const getTheLastWinner = true
    const game: Game = new Game("./__tests__/testInput.txt")
    const winnerScore = await game.play(getTheLastWinner)
    expect(winnerScore).toEqual(exampleFinalScore)
})