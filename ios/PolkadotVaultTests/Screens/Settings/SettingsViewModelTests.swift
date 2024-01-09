//
//  SettingsViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 08/01/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import XCTest

final class SettingsViewModelTests: XCTestCase {
    private var viewModel: SettingsView.ViewModel!
    private var onboardingMediatorMock: OnboardingMediatingMock!
    private var cancelBag: CancelBag!

    override func setUp() {
        super.setUp()
        cancelBag = CancelBag()
        onboardingMediatorMock = OnboardingMediatingMock()
        onboardingMediatorMock.underlyingOnboardingDone = Just(false).eraseToAnyPublisher()
        viewModel = SettingsView.ViewModel(onboardingMediator: onboardingMediatorMock)
    }

    override func tearDown() {
        viewModel = nil
        onboardingMediatorMock = nil
        cancelBag = nil
        super.tearDown()
    }

    func testLoadData() {
        // When
        viewModel.loadData()

        // Then
        XCTAssertEqual(viewModel.renderable, SettingsViewRenderable())
    }

    func testOnTapAction_Logs() {
        // Given
        let item: SettingsItem = .logs

        // When
        viewModel.onTapAction(item)

        // Then
        XCTAssertEqual(viewModel.detailScreen, item)
        XCTAssertTrue(viewModel.isDetailsPresented)
    }

    func testOnTapAction_Wipe() {
        // Given
        let item: SettingsItem = .wipe

        // When
        viewModel.onTapAction(item)

        // Then
        XCTAssertTrue(viewModel.isPresentingWipeConfirmation)
    }

    func testWipe() {
        // When
        viewModel.wipe()

        // Then
        XCTAssertEqual(onboardingMediatorMock.onboardVerifierRemovedCallsCount, 1)
        XCTAssertFalse(viewModel.isPresentingWipeConfirmation)
    }

    func testOnTapAction_TermsAndConditions() {
        // Given
        let item: SettingsItem = .termsAndConditions

        // When
        viewModel.onTapAction(item)

        // Then
        XCTAssertEqual(viewModel.detailScreen, item)
        XCTAssertTrue(viewModel.isDetailsPresented)
    }

    func testOnTapAction_PrivacyPolicy() {
        // Given
        let item: SettingsItem = .privacyPolicy

        // When
        viewModel.onTapAction(item)

        // Then
        XCTAssertEqual(viewModel.detailScreen, item)
        XCTAssertTrue(viewModel.isDetailsPresented)
    }

    func testOnTapAction_Networks() {
        // Given
        let item: SettingsItem = .networks

        // When
        viewModel.onTapAction(item)

        // Then
        XCTAssertEqual(viewModel.detailScreen, item)
        XCTAssertTrue(viewModel.isDetailsPresented)
    }

    func testOnTapAction_Verifier() {
        // Given
        let item: SettingsItem = .verifier

        // When
        viewModel.onTapAction(item)

        // Then
        XCTAssertEqual(viewModel.detailScreen, item)
        XCTAssertTrue(viewModel.isDetailsPresented)
    }
}
