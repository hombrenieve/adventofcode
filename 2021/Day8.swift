import Foundation





var instances = 0

while let input = readLine() {
    let output = input.components(separatedBy: "|")[1].components(separatedBy: " ")
    instances += output.filter({ $0.count == 2 ||
        $0.count == 3 ||
        $0.count == 4 ||
        $0.count == 7
    }).count
}

print("Total instances: \(instances)")
