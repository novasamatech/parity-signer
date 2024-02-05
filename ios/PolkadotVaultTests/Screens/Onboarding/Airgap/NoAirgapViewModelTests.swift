//
//  NoAirgapViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 02/02/2024.
//

import Foundation
@testable import PolkadotVault
import XCTest

final class NoAirgapViewModelTests: XCTestCase {
    private var viewModel: NoAirgapView.ViewModel!
    private var airgapMediatorMock: AirgapMediatingMock!
    private var onActionTapExecuted: Bool = false

    override func setUp() {
        super.setUp()
        airgapMediatorMock = AirgapMediatingMock()
        viewModel = NoAirgapView.ViewModel(
            mode: .onboarding,
            airgapMediator: airgapMediatorMock,
            onActionTap: { self.onActionTapExecuted = true }
        )
    }

    override func tearDown() {
        viewModel = nil
        airgapMediatorMock = nil
        onActionTapExecuted = false
        super.tearDown()
    }

    func testAirgapStatusUpdate_UpdatesCheckBoxState() {
        // Given
        let isAirplaneModeOn = true
        let isWifiOn = false

        // When
        airgapMediatorMock.startMonitoringAirgapReceivedUpdate.first?(isAirplaneModeOn, isWifiOn)

        // Then
        XCTAssertTrue(viewModel.isAirplaneModeChecked)
        XCTAssertTrue(viewModel.isWifiChecked)
    }

    func testToggleCheckbox_TogglesCheckboxState() {
        // Given
        let initialCheckboxState = viewModel.isCableCheckBoxSelected

        // When
        viewModel.toggleCheckbox()

        // Then
        XCTAssertNotEqual(viewModel.isCableCheckBoxSelected, initialCheckboxState)
    }

    func testOnDoneTap_ExecutesOnActionTap() {
        // When
        viewModel.onDoneTap()

        // Then
        XCTAssertTrue(onActionTapExecuted)
    }

    func testUpdateActionState_WhenAllConditionsMet_EnablesAction() {
        // Given
        airgapMediatorMock.startMonitoringAirgapReceivedUpdate.first?(true, false)
        viewModel.toggleCheckbox()

        // Then
        XCTAssertFalse(viewModel.isActionDisabled)
    }

    func testUpdateActionState_WhenNotAllConditionsMet_DisablesAction() {
        // Given
        airgapMediatorMock.startMonitoringAirgapReceivedUpdate.first?(true, true)

        // Then
        XCTAssertTrue(viewModel.isActionDisabled)
    }
}
