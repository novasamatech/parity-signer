//
//  TabTests.swift
//  NativeSignerTests
//
//  Created by Krzysztof Rodak on 18/08/2022.
//

@testable import NativeSigner
import XCTest

final class TabTests: XCTestCase {
    func test_init_whenBack_returnsNil() {
        // Given
        let footerButoon: FooterButton? = .back

        // When
        let result = Tab(footerButoon)

        // Then
        XCTAssertNil(result)
    }

    func test_init_whenNil_returnsNil() {
        // Given
        let footerButoon: FooterButton? = nil

        // When
        let result = Tab(footerButoon)

        // Then
        XCTAssertNil(result)
    }

    func test_init_whenLog_returnsLogs() {
        // Given
        let expectedResult: Tab = .logs
        let footerButoon: FooterButton? = .log

        // When
        let result = Tab(footerButoon)

        // Then
        XCTAssertEqual(result, expectedResult)
    }

    func test_init_whenScan_returnsScanner() {
        // Given
        let expectedResult: Tab = .scanner
        let footerButoon: FooterButton? = .scan

        // When
        let result = Tab(footerButoon)

        // Then
        XCTAssertEqual(result, expectedResult)
    }

    func test_init_whenKeys_returnsKeys() {
        // Given
        let expectedResult: Tab = .keys
        let footerButoon: FooterButton? = .keys

        // When
        let result = Tab(footerButoon)

        // Then
        XCTAssertEqual(result, expectedResult)
    }

    func test_init_whenSettings_returnsSettings() {
        // Given
        let expectedResult: Tab = .settings
        let footerButoon: FooterButton? = .settings

        // When
        let result = Tab(footerButoon)

        // Then
        XCTAssertEqual(result, expectedResult)
    }
}
