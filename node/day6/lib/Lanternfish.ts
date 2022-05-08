"use strict"

import { createReadStream } from "fs"
import * as readline from 'readline'
import { EventEmitter } from "stream"

export class Lanternfish {
    private _bus: EventEmitter = new EventEmitter()
    private timer: number = 0

    constructor(inputFileName: string) {
        const readStream = createReadStream(inputFileName)
        const reader = readline.createInterface(readStream)
        reader.on("line", (row: string) => {
            console.log(row)
        })
        reader.on('close', () => {
            this._bus.emit('ready')
        })
        reader.on('error', (error) => {
            console.error(error)
        })
    }

    count(days: number) {
        return days
    }
}