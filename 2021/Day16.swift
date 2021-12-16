import Foundation

extension String {
    func leftPadding(toLength: Int, withPad character: Character) -> String {
        let newLength = self.count
        if newLength < toLength {
            return String(repeatElement(character, count: toLength - newLength)) + self
        } else {
            return self.substring(from: index(self.startIndex, offsetBy: newLength - toLength))
        }
    }
}

struct Message {
    let data: [Character]
    var currentBit = 0
    var packet: Packet? = nil


    init(_ msg: String) {
        self.data = Array(msg)
        self.packet = buildPacket()
    }

    private mutating func buildLiteral(_ v: Int, _ t: Int) -> Packet {
        var reading = true
        var numberString = ""
        while reading {
            if data[currentBit].wholeNumberValue! == 0 {
                reading = false
            }
            numberString.append(String(data[currentBit+1...currentBit+4]))
            currentBit += 5
        }
        let number = Int(numberString, radix: 2)!
        let p = Literal(v, t, number)
        return p
    }

    private static func operatorFromCmd(_ v: Int, _ t: Int) -> Operator {
        switch(t) {
            case 0:
                return Sum(v, t)
            case 1:
                return Product(v, t)
            case 2:
                return Min(v, t)
            case 3:
                return Max(v, t)
            case 5:
                return Gt(v, t)
            case 6:
                return Lt(v, t)
            case 7:
                return Eq(v, t)
            default:
                return Operator(v, t)
        }
    }

    private mutating func buildOperator(_ v: Int, _ t: Int) -> Packet {
        let i = data[currentBit].wholeNumberValue!
        currentBit += 1
        if i == 0 {
            let length = Int(String(data[currentBit...currentBit+14]), radix: 2)!
            currentBit += 15
            let origin = currentBit
            let p = Message.operatorFromCmd(v, t)
            while length != currentBit-origin {
                p.subPackets.append(buildPacket())
            }
            return p
        } else {
            let count = Int(String(data[currentBit...currentBit+10]), radix: 2)!
            currentBit += 11
            let p = Message.operatorFromCmd(v, t)
            for _ in 1...count {
                p.subPackets.append(buildPacket())
            }
            return p
        }        
    }

    private mutating func buildPacket() -> Packet {
        let v = Int(String(data[currentBit...currentBit+2]), radix: 2)!
        let t = Int(String(data[currentBit+3...currentBit+5]), radix: 2)!
        currentBit += 6
        switch(t) {
            case 4:
                return buildLiteral(v, t)
            default:
                return buildOperator(v, t)
        }
    }
}


class Packet {
    let version: Int
    let type: Int    

    init(_ v: Int, _ t: Int) {
        self.version = v
        self.type = t        
    }

    func resolve() -> Int {
        return 0
    }
}

class Literal: Packet {
    var literal: Int = 0

    init(_ v: Int, _ t: Int, _ literal: Int) {
        super.init(v, t)
        self.literal = literal
    }    

    override func resolve() -> Int {
        return literal
    }
}

class Operator: Packet {
    var subPackets: [Packet] = []
}

class Sum: Operator {
    override func resolve() -> Int {
        var sum = 0
        for p in self.subPackets {
            sum += p.resolve()
        }
        return sum
    }
}

class Product: Operator {
    override func resolve() -> Int {
        var prod = 1
        for p in self.subPackets {
            prod *= p.resolve()
        }
        return prod
    }
}

class Min: Operator {
    override func resolve() -> Int {
        return self.subPackets.map({ $0.resolve() }).min()!
    }
}

class Max: Operator {
    override func resolve() -> Int {
        return self.subPackets.map({ $0.resolve() }).max()!
    }
}

class Gt: Operator {
    override func resolve() -> Int {
        return self.subPackets[0].resolve() > self.subPackets[1].resolve() ? 1 : 0
    }
}

class Lt: Operator {
    override func resolve() -> Int {
        return self.subPackets[0].resolve() < self.subPackets[1].resolve() ? 1 : 0
    }
}

class Eq: Operator {
    override func resolve() -> Int {
        return self.subPackets[0].resolve() == self.subPackets[1].resolve() ? 1 : 0
    }
}

func convertToBinary(hex: String) -> String {
    var result = ""
    for el in hex {
        result.append(String(Int(String(el), radix: 16)!, radix: 2).leftPadding(toLength: 4, withPad: "0"))
    }
    return result
}


let msg = Message(convertToBinary(hex: readLine()!))

print("Result: \(msg.packet!.resolve())")
