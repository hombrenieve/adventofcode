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
    var packets: [Packet] = []


    init(_ msg: String) {
        self.data = Array(msg)
        while let p = buildPacket() {
            packets.append(p)
        }
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
        let p = Packet(v, t)
        p.literal = number
        return p
    }

    private mutating func buildOperator(_ v: Int, _ t: Int) -> Packet {
        let i = data[currentBit].wholeNumberValue!
        currentBit += 1
        if i == 0 {
            let length = Int(String(data[currentBit...currentBit+14]), radix: 2)!
            currentBit += 15
            let origin = currentBit
            let p = Packet(v, t)
            while length != currentBit-origin {
                p.subPackets.append(buildPacket()!)
            }
            return p
        } else {
            let count = Int(String(data[currentBit...currentBit+10]), radix: 2)!
            currentBit += 11
            let p = Packet(v, t)
            for _ in 1...count {
                p.subPackets.append(buildPacket()!)
            }
            return p
        }        
    }

    private mutating func buildPacket() -> Packet? {
        if data.count-currentBit < 11 {
            return nil
        }
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
    var literal: Int? = nil
    var subPackets: [Packet] = []

    init(_ v: Int, _ t: Int) {
        self.version = v
        self.type = t        
    }
}

func sumV(accum: Int, packets: [Packet]) -> Int {
    if packets.count == 0 {
        return accum
    }
    var local = accum+packets[0].version
    local += sumV(accum: 0, packets: packets[0].subPackets)
    return sumV(accum: local, packets: Array(packets.dropFirst()))
}

func convertToBinary(hex: String) -> String {
    var result = ""
    for el in hex {
        result.append(String(Int(String(el), radix: 16)!, radix: 2).leftPadding(toLength: 4, withPad: "0"))
    }
    return result
}


let msg = Message(convertToBinary(hex: readLine()!))

let sum = sumV(accum: 0, packets: msg.packets)

print("Result: \(sum)")
