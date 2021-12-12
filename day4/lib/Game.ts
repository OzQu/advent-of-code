"use strict"

import { createReadStream } from 'fs'
import * as readline from 'readline'
import { EventEmitter } from 'stream'
import { Board } from './Board'

enum ReadPhase {
    ReadDrawnNumbers,
    ReadBoardNumbers,
    ChangeBoard
}

export class Game {
    private _inputNumbers: number[]
    private _boards: Board[]
    private _bus: EventEmitter = new EventEmitter()
    private _winners: { boardId: number; winScore: number }[]

    constructor(inputFileName: string) {
        this._inputNumbers = []
        this._boards = []
        this._winners = []

        const readStream = createReadStream(inputFileName)
        const reader = readline.createInterface(readStream)
        let phase: ReadPhase = ReadPhase.ReadDrawnNumbers
        reader.on("line", (line: string) => {
            if (phase === ReadPhase.ReadDrawnNumbers) {
                if (line.length === 0) {
                    phase = ReadPhase.ChangeBoard
                    return
                }
                this._inputNumbers = line.split(',').map((value: string) => parseInt(value))
                console.log(`Read drawn numbers: ${line}`)
                return
            }
            if (phase === ReadPhase.ChangeBoard) {
                if (line.length === 0) return
                this._boards.push(new Board(this._boards.length))
                phase = ReadPhase.ReadBoardNumbers
            }
            if (phase === ReadPhase.ReadBoardNumbers) {
                if (line.length === 0) {
                    phase = ReadPhase.ChangeBoard
                    console.log("Change board")
                    return
                }
                this._boards[this._boards.length - 1].addNewRow(line)
                return
            }
        })
        reader.on('close', () => {
            console.log(`Handled ${this._boards.length} boards`)
            this._bus.emit('ready')
        })
        reader.on('error', (error) => {
            console.error(error)
        })
    }

    private callNumberToBoards(): void {
        const value = this._inputNumbers.shift()
        console.log(`Call number ${value} to boards`)
        for (let boardIndex = 0; boardIndex < this._boards.length; boardIndex++) {
            const board = this._boards[boardIndex]
            if (board.callNumber(value)) {
                console.log(`Board id ${board.id} has won`)
                let winScore = board.calculateScore(value)
                const alreadyAWinnerIndex = this._winners.findIndex(winner => winner.boardId === board.id)
                if (alreadyAWinnerIndex !== -1) {
                    this._winners[alreadyAWinnerIndex].winScore = winScore
                } else {
                    this._winners.push({ boardId: board.id, winScore: winScore})
                }
            }
        }
    } 

    async play(getTheLastWinner: boolean = false) {
        await new Promise(resolve => this._bus.once('ready', resolve))
        console.log(`Game on with numbers ${this._inputNumbers}`)
        if (this._boards.length === 0) throw Error("No boards")
        let winnerScore: Number | undefined = undefined
        while (this._inputNumbers.length > 0) {
            this.callNumberToBoards()
            if (this._winners.length > 0 && !getTheLastWinner) {
                winnerScore = this._winners.shift()?.winScore
                break
            }
            if (getTheLastWinner && this._winners.length === this._boards.length) {
                winnerScore = this._winners.pop()?.winScore
                break
            }
        }
        return new Promise(resolve => resolve(winnerScore))
    }
}