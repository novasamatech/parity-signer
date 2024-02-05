//
//  AirgapComponentTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 31/01/2024.
//

@testable import PolkadotVault
import SwiftUI
import XCTest

final class AirgapComponentTests: XCTestCase {
    func testTitleForAiplaneMode() {
        XCTAssertEqual(AirgapComponent.aiplaneMode.title, Localizable.Airgap.Label.airplane.string)
    }

    func testTitleForWifi() {
        XCTAssertEqual(AirgapComponent.wifi.title, Localizable.Airgap.Label.wifi.string)
    }
}
