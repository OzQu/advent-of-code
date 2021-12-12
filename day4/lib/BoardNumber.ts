"use strict"

export class BoardNumber {
    private _number: number
    private _called: boolean

    constructor(number: number) {
        this._number = number
        this._called = false
    }

    public get number(): number {
        return this._number
    }
    public get called(): boolean {
        return this._called
    }
    public set called(value: boolean) {
        this._called = value
    }

}