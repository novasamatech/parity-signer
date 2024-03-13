//
//  NoAirgapViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 02/02/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import XCTest

final class NoAirgapViewModelTests: XCTestCase {
    private var viewModel: NoAirgapView.ViewModel!
    private var airgapMediatorMock: AirgapMediatingMock!
    private var cancellables: Set<AnyCancellable> = []
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
        cancellables = []
        super.tearDown()
    }

    func testAirgapStatusUpdate_UpdatesCheckBoxState() {
        // Given
        let expectedState = AirgapState(isAirplaneModeOn: true, isWifiOn: false, isLocationServiceEnabled: false)
        airgapMediatorMock.simulateAirgapState(expectedState)

        let expectation = XCTestExpectation(description: "Receive AirgapState Update")

        // When
        viewModel.$isAirplaneModeChecked
            .dropFirst()
            .sink { isAirplaneModeOn in
                XCTAssertTrue(isAirplaneModeOn)
                expectation.fulfill()
            }
            .store(in: &cancellables)

        viewModel.$isWifiChecked
            .dropFirst()
            .sink { isWifiOn in
                XCTAssertTrue(isWifiOn)
                expectation.fulfill()
            }
            .store(in: &cancellables)

        // Then
        wait(for: [expectation], timeout: 1.0)
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
        airgapMediatorMock.simulateAirgapState(AirgapState(
            isAirplaneModeOn: true,
            isWifiOn: false,
            isLocationServiceEnabled: false
        ))

        // When
        viewModel.toggleCheckbox()

        let expectation = XCTestExpectation(description: "Wait for updates")
        DispatchQueue.main.async {
            expectation.fulfill()
        }
        wait(for: [expectation], timeout: 1.0)

        // Then
        XCTAssertFalse(viewModel.isActionDisabled)
    }

    func testUpdateActionState_WhenNotAllConditionsMet_DisablesAction() {
        // Given
        airgapMediatorMock.simulateAirgapState(AirgapState(
            isAirplaneModeOn: false,
            isWifiOn: true,
            isLocationServiceEnabled: true
        ))

        // When
        let expectation = XCTestExpectation(description: "Wait for updates")
        DispatchQueue.main.async {
            expectation.fulfill()
        }
        wait(for: [expectation], timeout: 1.0)

        // Then
        XCTAssertTrue(viewModel.isActionDisabled)
    }
}

// MARK: - Mocks

final class AirgapMediatingMock: AirgapMediating {
    private let airgapSubject = PassthroughSubject<AirgapState, Never>()
    private let isConnectedSubject = PassthroughSubject<Bool, Never>()

    var isConnectedPublisher: AnyPublisher<Bool, Never> {
        isConnectedSubject.eraseToAnyPublisher()
    }

    var airgapPublisher: AnyPublisher<AirgapState, Never> {
        airgapSubject.eraseToAnyPublisher()
    }

    var startMonitoringAirgapCallsCount = 0

    func startMonitoringAirgap() {
        startMonitoringAirgapCallsCount += 1
    }

    // Helper methods to simulate updates
    func simulateIsConnected(_ isConnected: Bool) {
        isConnectedSubject.send(isConnected)
    }

    func simulateAirgapState(_ state: AirgapState) {
        airgapSubject.send(state)
    }
}
