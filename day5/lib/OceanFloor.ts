"use strict"

import { createReadStream } from "fs"
import * as readline from 'readline'
import { EventEmitter } from "stream"

enum MovementType {
    HORIZONTAL,
    VERTICAL,
    DIAGONAL,
    DIAGONAL45
}
type Coordinate = { 
    x: number, 
    y: number
}
type DataMap = Record<number, Record<number, number>>

class Line {
    private _startCoord: Coordinate
    private _endCoord: Coordinate
    constructor(startCoord: Coordinate, endCoord: Coordinate) {
        this._startCoord = startCoord
        this._endCoord = endCoord
    }
    public get startCoord(): Coordinate {
        return this._startCoord
    }
    public set startCoord(value: Coordinate) {
        this._startCoord = value
    }
    public get endCoord(): Coordinate {
        return this._endCoord
    }
    public set endCoord(value: Coordinate) {
        this._endCoord = value
    }
}

export class OceanFloor {
    private _bus: EventEmitter = new EventEmitter()
    /*     
    lines 1,1 -> 1,2 and 1,1 -> 1,3 and 2,2 -> 2,3 would result
    {
    1: {
            1: 2,
            2: 2,
            3: 1
        },
    2:  {
            2: 1,
            3: 1
        }
    }
    */
    private _oceanFloorMap: DataMap = {}

    constructor(
        inputFileName: string, 
        filterDiagonals: boolean = false,
        include45Diagonals: boolean = false) {
        const readStream = createReadStream(inputFileName)
        const reader = readline.createInterface(readStream)
        reader.on("line", (row: string) => {
            const values = row.match(/(\d+,\d+)/g)
            if (values?.length !== 2) throw Error(`Could not parse values ${values}`)
            const startValues: number[] = values[0].split(",").map((value) => parseInt(value))
            const endValues: number[] = values[1].split(",").map((value) => parseInt(value))
            if (startValues.length !== 2) throw Error(`Could not parse start ${startValues}`)
            if (endValues.length !== 2) throw Error(`Could not parse end ${endValues}`)

            const line = new Line(
                { x: startValues[0], y: startValues[1] },
                { x: endValues[0], y: endValues[1] })
            this.addLine(line, filterDiagonals, include45Diagonals)
        })
        reader.on('close', () => {
            // console.log("Map is: ", this._oceanFloorMap)
            this._bus.emit('ready')
        })
        reader.on('error', (error) => {
            console.error(error)
        })
    }
    private moveCoordinate(deltaX: number, deltaY: number, coordinate: Coordinate): Coordinate {
        if (deltaX === 0) {
            // Vertical movement
            if (deltaY > 0) return {x: coordinate.x, y: ++coordinate.y}
            if (deltaY < 0) return {x: coordinate.x, y: --coordinate.y}
        }
        if (deltaY === 0) {
            // Horizontal movement
            if (deltaX > 0) return {x: ++coordinate.x, y: coordinate.y}
            if (deltaX < 0) return {x: --coordinate.x, y: coordinate.y}
        }
        if (Math.abs(deltaY) === Math.abs(deltaX)) {
            // 45 diagonal movement
            return {
                x: deltaX > 0 ? ++coordinate.x : --coordinate.x,
                y: deltaY > 0 ? ++coordinate.y : --coordinate.y,
            }
        }
        if (Math.abs(deltaY) > Math.abs(deltaX)) {
            return coordinate
            // if (deltaY > 0) coordinate.y += 1
            // if (deltaY < 0) coordinate.y -= 1
        }
        if (Math.abs(deltaY) < Math.abs(deltaX)) {
            return coordinate
            // if (deltaX > 0) coordinate.x += 1
            // if (deltaX < 0) coordinate.x -= 1
        }
        return coordinate
    }
    private getMovementType(deltaX: number, deltaY: number): MovementType {
        if (deltaX === 0) return MovementType.VERTICAL
        if (deltaY === 0) return MovementType.HORIZONTAL
        if (Math.abs(deltaY) === Math.abs(deltaX)) return MovementType.DIAGONAL45
        return MovementType.DIAGONAL
    }
    private addLine(line: Line, filterDiagonals: boolean, include45Diagonals: boolean) {
        filterDiagonals === include45Diagonals
        let drawCoord: Coordinate = {...line.startCoord}
        const deltaY: number = line.endCoord.y - line.startCoord.y
        const deltaX: number = line.endCoord.x - line.startCoord.x
        const movementType: MovementType = this.getMovementType(deltaY, deltaX)
        if (filterDiagonals && !include45Diagonals 
            && (movementType !== MovementType.HORIZONTAL && movementType !== MovementType.VERTICAL)) return
        if (filterDiagonals && include45Diagonals && movementType === MovementType.DIAGONAL) return

        while(!(drawCoord.x === line.endCoord.x && drawCoord.y === line.endCoord.y)) {
            // Save Coordinate to map
            if (this._oceanFloorMap[drawCoord.x] === undefined) this._oceanFloorMap[drawCoord.x] = {}
            this._oceanFloorMap[drawCoord.x][drawCoord.y] = ++this._oceanFloorMap[drawCoord.x][drawCoord.y] || 1
            // Move to next coordinate
            drawCoord = this.moveCoordinate(deltaX, deltaY, drawCoord)
        }
        if (this._oceanFloorMap[drawCoord.x] === undefined) this._oceanFloorMap[drawCoord.x] = {}
        this._oceanFloorMap[drawCoord.x][drawCoord.y] = ++this._oceanFloorMap[drawCoord.x][drawCoord.y] || 1
    }

    
    async count(limit: number): Promise<number> {
        await new Promise(resolve => this._bus.once('ready', resolve))
        let count: number = 0
        for (const xCoord in this._oceanFloorMap) {
            if (Object.prototype.hasOwnProperty.call(this._oceanFloorMap, xCoord)) {
                const xCoordMap: Record<number, number> = this._oceanFloorMap[xCoord];
                for (const yCoord in xCoordMap) {
                    if (Object.prototype.hasOwnProperty.call(xCoordMap, yCoord)) {
                        const yCount: number = xCoordMap[yCoord];
                        if (yCount >= limit) count++
                    }
                }
            }
        }
        return count
    }
}