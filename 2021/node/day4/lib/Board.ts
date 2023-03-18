"use strict"

import { BoardNumber } from './BoardNumber'

export class Board {
    private _rows: Array<Array<BoardNumber>>
    private _id: number
    constructor(id: number) {
        this._rows = []
        this._id = id
    }
    public get id(): number {
        return this._id
    }
    addNewRow(rowContent: string) {
        const rowValues: number[] | undefined = rowContent.match(/(\d{1,2})/g)?.map((strValue) => parseInt(strValue))
        if (rowValues === undefined) throw Error('Parsing failed')
        this._rows.push(rowValues.map(value => new BoardNumber(value)))
        console.log(`Board ${this._id}: New row ${rowValues}`)
    }
    callNumber(value: number | undefined) : boolean {
        if (value === undefined) throw Error('Undefined number check')

        for (let rowIndex = 0; rowIndex < this._rows.length; ++rowIndex) {
            for (let columnIndex = 0; columnIndex < this._rows[rowIndex].length; columnIndex++) {
                const boardNumber = this._rows[rowIndex][columnIndex];
                if (boardNumber.number === value) {
                    boardNumber.called = true
                }
            }
        }
        return this.checkRows() || this.checkColumns()
    }
    checkRows() : boolean {
        const foo = this._rows.filter(row => {
            return row.every(boardNumber => boardNumber.called)
        })
        const bar = foo.length > 0
        return bar
    }
    checkColumns() : boolean {
        for (let columnIndex = 0; columnIndex < this._rows[0].length; columnIndex++) {
            const columnCalled = this._rows.every(row => {
                return row[columnIndex].called
            })
            if (columnCalled) return true
        }
        return false
    }
    calculateScore(called: number | undefined): number {
        if (called === undefined) throw Error('Last called number was undefined')
        return called * this.sumUnmarkedNumbers()
    }
    sumUnmarkedNumbers(): number {
        return this._rows.reduce((sum: number, currRow: BoardNumber[]) => {
            return sum + currRow.reduce((sum: number, currBoardNumber: BoardNumber) => {
                return sum + (!currBoardNumber.called ? currBoardNumber.number : 0)
            }, 0)
        }, 0)
    }
}