//
//  KeySetListViewModelBuilderTests.swift
//  NativeSignerTests
//
//  Created by Krzysztof Rodak on 22/08/2022.
//

@testable import NativeSigner
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
        let identicon: [UInt8] = [123]
        let seedNameCard = SeedNameCard(seedName: name, identicon: .svg(image: identicon), derivedKeysCount: 0)
        let expectedResult = KeySetListViewModel(
            list: [KeySetViewModel(seed: seedNameCard, keyName: name, derivedKeys: derivedKeys, identicon: identicon)]
        )

        // When
        let result = subject.build(for: MSeeds(seedNameCards: [seedNameCard]))

        // Then
        XCTAssertEqual(result, expectedResult)
    }

    func test_build_singleDerivedKey_returnsExpectedModel() {
        // Given
        let name = "name"
        let derivedKeys = "1 Key"
        let identicon: [UInt8] = [123]
        let seedNameCard = SeedNameCard(seedName: name, identicon: .svg(image: identicon), derivedKeysCount: 1)
        let expectedResult = KeySetListViewModel(
            list: [KeySetViewModel(seed: seedNameCard, keyName: name, derivedKeys: derivedKeys, identicon: identicon)]
        )

        // When
        let result = subject.build(for: MSeeds(seedNameCards: [seedNameCard]))

        // Then
        XCTAssertEqual(result, expectedResult)
    }

    func test_build_multipleDerivedKeys_returnsExpectedModel() {
        // Given
        let name = "name"
        let derivedKeys = "3 Keys"
        let identicon: [UInt8] = [123]
        let seedNameCard = SeedNameCard(seedName: name, identicon: .svg(image: identicon), derivedKeysCount: 3)
        let expectedResult = KeySetListViewModel(
            list: [KeySetViewModel(seed: seedNameCard, keyName: name, derivedKeys: derivedKeys, identicon: identicon)]
        )

        // When
        let result = subject.build(for: MSeeds(seedNameCards: [seedNameCard]))

        // Then
        XCTAssertEqual(result, expectedResult)
    }
}
