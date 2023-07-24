//
//  PublicKeyDecoderTests.swift
//
//
//  Created by Krzysztof Rodak on 23/07/2023.
//

import Foundation

@testable import PolkadotIdenticon
import XCTest

final class PublicKeyDecoderTests: XCTestCase {
    private var publicKeyDecoder: PublicKeyDecoder!

    private let expectedOutput: [UInt8] = [
        212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88,
        133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125
    ]

    override func setUp() {
        super.setUp()
        publicKeyDecoder = PublicKeyDecoder()
    }

    override func tearDown() {
        publicKeyDecoder = nil
        super.tearDown()
    }

    func testDecodeData() {
        // Given
        let dataPublicKey: PublicKey = .data(Data(expectedOutput))

        // When
        let result = publicKeyDecoder.decode(from: dataPublicKey)

        // Then
        XCTAssertEqual(result, expectedOutput)
    }

    func testDecodeHex() {
        // Given
        let hexPublicKey: PublicKey = .hex("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d")

        // When
        let result = publicKeyDecoder.decode(from: hexPublicKey)

        // Then
        XCTAssertEqual(result, expectedOutput)
    }

    func testDecodeBase58() {
        // Given
        let base58PublicKey: PublicKey = .base58("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")

        // When
        let result = publicKeyDecoder.decode(from: base58PublicKey)

        // Then
        XCTAssertEqual(result, expectedOutput)
    }
}
