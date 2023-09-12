//
//  TabViewModelBuilderTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 18/08/2022.
//

@testable import PolkadotVault
import XCTest

final class TabViewModelBuilderTests: XCTestCase {
    private var subject: TabViewModelBuilder!

    override func setUp() {
        super.setUp()
        subject = TabViewModelBuilder()
    }

    func test_build_keys_returnsExpectedModel() {
        // Given
        let tab: Tab = .keys
        let isSelected = false
        let expectedResult = TabViewModel(
            action: .navbarKeys,
            isActive: isSelected,
            icon: Asset.tabbarKeys.swiftUIImage,
            label: Localizable.TabBar.keys.text,
            tab: tab
        )

        // When
        let result = subject.build(for: tab, isSelected: isSelected)

        // Then
        XCTAssertEqual(result, expectedResult)
    }

    func test_build_scanner_returnsExpectedModel() {
        // Given
        let tab: Tab = .scanner
        let isSelected = false
        let expectedResult = TabViewModel(
            action: nil,
            isActive: isSelected,
            icon: Asset.tabbarScanner.swiftUIImage,
            label: Localizable.TabBar.scanner.text,
            tab: tab
        )

        // When
        let result = subject.build(for: tab, isSelected: isSelected)

        // Then
        XCTAssertEqual(result, expectedResult)
    }
}
