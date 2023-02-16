//
//  NavigationTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

@testable import PolkadotVault
import XCTest

final class NavigationTests: XCTestCase {
    func test_init_whenNilPassed_setsEmptyString() {
        // When
        let result = Navigation(action: .goBack, details: nil, seedPhrase: nil)

        // Then
        XCTAssertEqual(result.details, "")
        XCTAssertEqual(result.seedPhrase, "")
    }
}
