//
//  NetworkSettingDetailsActionModalViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 10/01/2024.
//

import Foundation
@testable import PolkadotVault
import SwiftUI
import XCTest

final class NetworkSettingsDetailsActionModalViewModelTests: XCTestCase {
    private var isPresented: Bool = false
    private var shouldPresentRemoveNetworkConfirmation: Bool = false
    private var shouldSignSpecs: Bool = false

    private func createBinding(for keyPath: WritableKeyPath<NetworkSettingsDetailsActionModalViewModelTests, Bool>)
        -> Binding<Bool> {
        let defaultValue = self[keyPath: keyPath]
        return .init(get: { [weak self] in
            self?[keyPath: keyPath] ?? defaultValue
        }, set: { [weak self] in
            self?[keyPath: keyPath] = $0
        })
    }

    func testToggleSignSpecs() {
        // Given
        let viewModel = NetworkSettingsDetailsActionModal.ViewModel(
            isPresented: createBinding(for: \.isPresented),
            shouldPresentRemoveNetworkConfirmation: createBinding(for: \.shouldPresentRemoveNetworkConfirmation),
            shouldSignSpecs: createBinding(for: \.shouldSignSpecs)
        )
        let expectation = expectation(description: "AnimationDelay")

        // When
        viewModel.toggleSignSpecs()

        // Then
        DispatchQueue.main.asyncAfter(deadline: .now() + 1) {
            XCTAssertTrue(self.shouldSignSpecs)
            XCTAssertFalse(self.isPresented)
            expectation.fulfill()
        }

        waitForExpectations(timeout: 2, handler: nil)
    }

    func testToggleRemoveNetworkConfirmation() {
        // Given
        let viewModel = NetworkSettingsDetailsActionModal.ViewModel(
            isPresented: createBinding(for: \.isPresented),
            shouldPresentRemoveNetworkConfirmation: createBinding(for: \.shouldPresentRemoveNetworkConfirmation),
            shouldSignSpecs: createBinding(for: \.shouldSignSpecs)
        )
        let expectation = expectation(description: "AnimationDelay")

        // When
        viewModel.toggleRemoveNetworkConfirmation()

        // Then
        DispatchQueue.main.asyncAfter(deadline: .now() + 1) {
            XCTAssertTrue(self.shouldPresentRemoveNetworkConfirmation)
            XCTAssertFalse(self.isPresented)
            expectation.fulfill()
        }

        waitForExpectations(timeout: 2, handler: nil)
    }

    func testDismissActionSheet() {
        // Given
        let viewModel = NetworkSettingsDetailsActionModal.ViewModel(
            isPresented: createBinding(for: \.isPresented),
            shouldPresentRemoveNetworkConfirmation: createBinding(for: \.shouldPresentRemoveNetworkConfirmation),
            shouldSignSpecs: createBinding(for: \.shouldSignSpecs)
        )
        let expectation = expectation(description: "AnimationDelay")

        // When
        viewModel.dismissActionSheet()

        // Then
        DispatchQueue.main.asyncAfter(deadline: .now() + 1) {
            XCTAssertFalse(self.isPresented)
            expectation.fulfill()
        }

        waitForExpectations(timeout: 2, handler: nil)
    }
}
