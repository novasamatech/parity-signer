//
//  NavigationTests.swift
//  NativeSignerTests
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

@testable import NativeSigner
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
