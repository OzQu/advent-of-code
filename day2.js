"use strict"

const values = require("./day2InputValues")

class Point {
    // h_pos = undefined
    // depth = undefined
    constructor(h_pos, depth) {
        this.location = {
            h_pos: h_pos,
            depth: depth
        }
    }
    get location() {
        return this.locationValue
    }
    set location(location) {
        this.locationValue = location
    }
    setLocationAttr(key, value) {
        this.location[key] = value
    }

    get sum() {
        return this.location.h_pos * this.location.depth
    }

    handleCommand = (command) => {
        const cmdVal = command.split(" ")
        const cmd = cmdVal[0]
        const val = parseInt(cmdVal[1])
        this.move[cmd](val)
    }

    move = {
        forward: (amount) => {
            this.location.h_pos += amount
        },
        down: (amount) => {
            this.location.depth += amount
        },
        up: (amount) => {
            this.location.depth -= amount
        }
    }

    static handleValues(values, startingPoint) {
        // handle all values.
        values.reduce((prev, next) => {
            prev.handleCommand(next)
            return prev
        }, startingPoint)
        return startingPoint
    }
}
   
let point = new Point(0,0)
Point.handleValues(values, point)
console.log(point.location)
// part 1
console.log(point.sum)

class PointV2 extends Point {
    aim = undefined
    constructor(h_pos, depth, aim) {
        super(h_pos, depth)
        this.setLocationAttr("aim", aim)
    }

    move = {
        forward: (amount) => {
            this.location.h_pos += amount
            this.location.depth += (amount * this.location.aim)
        },
        down: (amount) => {
            this.location.aim += amount
        },
        up: (amount) => {
            this.location.aim -= amount
        }
    }
}

let pointV2 = new PointV2(0,0,0)
PointV2.handleValues(values, pointV2)
console.log(pointV2.location)
console.log(pointV2.sum)

