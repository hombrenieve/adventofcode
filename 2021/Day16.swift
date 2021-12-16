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


class Packet {
    let version: Int
    let type: Int
    let literal: Int?
    let next: Packet?
    let subPackets: [Packet]

    init(binary: String) {
        let asArray = Array(binary)
        self.version = Int(String(asArray[0...2]), radix: 2)!
        self.type = Int(String(asArray[2...4]), radix: 2)!
        if self.type == 4 {
            var readingLiteral
        }
    }
}

func convertToBinary(hex: String) -> String {
    var result = ""
    for el in hex {
        result.append(String(Int(String(el), radix: 16)!, radix: 2).leftPadding(toLength: 4, withPad: "0"))
    }
    return result
}


let bs = convertToBinary(hex: readLine()!)
let pk = Packet(binary: bs)
   