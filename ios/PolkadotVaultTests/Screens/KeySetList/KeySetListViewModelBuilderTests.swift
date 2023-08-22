//
//  KeySetListViewModelBuilderTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 22/08/2022.
//

@testable import PolkadotVault
import XCTest

final class KeySetListViewModelBuilderTests: XCTestCase {
    private var subject: KeySetListViewModelBuilder!

    override func setUp() {
        super.setUp()
        subject = KeySetListViewModelBuilder()
    }

    func test_build_noDerivedKeys_returnsExpectedModel() {
        // Given
        let name = "name"
        let derivedKeys: String? = nil
        let identicon = Identicon.stubIdenticon
        let seedNameCard = SeedNameCard(
            seedName: name,
            identicon: identicon,
            usedInNetworks: ["polkadot", "kusama", "westend"],
            derivedKeysCount: 0
        )

        // When
        let result = subject.build(for: MSeeds(seedNameCards: [seedNameCard]))

        // Then
        XCTAssertEqual(result.list.first?.seed, seedNameCard)
        XCTAssertEqual(result.list.first?.keyName, name)
        XCTAssertEqual(result.list.first?.derivedKeys, derivedKeys)
        XCTAssertEqual(result.list.first?.identicon, identicon)
    }

    func test_build_singleDerivedKey_returnsExpectedModel() {
        // Given
        let name = "name"
        let derivedKeys = "1 Key"
        let identicon = Identicon.stubIdenticon
        let seedNameCard = SeedNameCard(
            seedName: name,
            identicon: identicon,
            usedInNetworks: ["polkadot", "kusama", "westend"],
            derivedKeysCount: 1
        )

        // When
        let result = subject.build(for: MSeeds(seedNameCards: [seedNameCard]))

        // Then
        XCTAssertEqual(result.list.first?.seed, seedNameCard)
        XCTAssertEqual(result.list.first?.keyName, name)
        XCTAssertEqual(result.list.first?.derivedKeys, derivedKeys)
        XCTAssertEqual(result.list.first?.identicon, identicon)
    }

    func test_build_multipleDerivedKeys_returnsExpectedModel() {
        // Given
        let name = "name"
        let derivedKeys = "3 Keys"
        let identicon = Identicon.stubIdenticon
        let seedNameCard = SeedNameCard(
            seedName: name,
            identicon: identicon,
            usedInNetworks: ["polkadot", "kusama", "westend"],
            derivedKeysCount: 3
        )

        // When
        let result = subject.build(for: MSeeds(seedNameCards: [seedNameCard]))

        // Then
        XCTAssertEqual(result.list.first?.seed, seedNameCard)
        XCTAssertEqual(result.list.first?.keyName, name)
        XCTAssertEqual(result.list.first?.derivedKeys, derivedKeys)
        XCTAssertEqual(result.list.first?.identicon, identicon)
    }
}
