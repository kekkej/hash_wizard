#!/usr/bin/env swift

import Foundation
import CryptoKit

guard CommandLine.arguments.count == 3 else {
    fputs("Usage: SwiftHasher.swift <algorithm> <input>\n", stderr)
    exit(1)
}

let algo = CommandLine.arguments[1].lowercased()
let input = CommandLine.arguments[2]
let data = Data(input.utf8)

func printHash<T: Digest>(_ digest: T) {
    let hex = digest.map { String(format: "%02x", $0) }.joined()
    print(hex)
}

switch algo {
case "sha256":
    printHash(SHA256.hash(data: data))
case "sha1":
    printHash(Insecure.SHA1.hash(data: data))
case "md5":
    printHash(Insecure.MD5.hash(data: data))
default:
    fputs("Unsupported algorithm: \(algo)\n", stderr)
    exit(2)
}
