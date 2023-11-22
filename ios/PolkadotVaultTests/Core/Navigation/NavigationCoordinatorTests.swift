//
//  NavigationCoordinatorTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

@testable import PolkadotVault
import XCTest

final class NavigationCoordinatorTests: XCTestCase {
    private var backendActionPerformer: BackendNavigationPerformingMock!
    private var subject: NavigationCoordinator!

    override func setUp() {
        super.setUp()
        backendActionPerformer = BackendNavigationPerformingMock()
        backendActionPerformer.performBackendReturnValue = .success(.generate())
        subject = NavigationCoordinator(
            backendActionPerformer: backendActionPerformer
        )
    }

    func test_performNavigation_callsBackendPerformerWithExpectedData() {
        // Given
        let expectedAction = Action.goBack
        let expectedDetails = "details"
        let expectedSeedPhrase = "seedPhrase"
        let navigation = Navigation(action: expectedAction, details: expectedDetails, seedPhrase: expectedSeedPhrase)

        // When
        subject.performFake(navigation: navigation)

        // Then
        XCTAssertEqual(backendActionPerformer.performBackendActionCallsCount, 1)
        XCTAssertEqual(backendActionPerformer.performBackendReceivedAction, [expectedAction])
        XCTAssertEqual(backendActionPerformer.performBackendReceivedDetails, [expectedDetails])
        XCTAssertEqual(backendActionPerformer.performBackendReceivedSeedPhrase, [expectedSeedPhrase])
    }

    func test_performNavigation_whenDebounceIsFinished_backendCanPerformAnotherAction() {
        // Given
        let firstNavigation = Navigation(action: .goBack, details: "details", seedPhrase: nil)
        let secondNavigation = Navigation(action: .goForward, details: nil, seedPhrase: "seed")

        // When
        subject.performFake(navigation: firstNavigation)
        subject.performFake(navigation: secondNavigation)

        // Then
        XCTAssertEqual(backendActionPerformer.performBackendActionCallsCount, 2)
        XCTAssertEqual(
            backendActionPerformer.performBackendReceivedAction,
            [firstNavigation.action, secondNavigation.action]
        )
        XCTAssertEqual(backendActionPerformer.performBackendReceivedDetails, [firstNavigation.details, ""])
        XCTAssertEqual(backendActionPerformer.performBackendReceivedSeedPhrase, ["", secondNavigation.seedPhrase])
    }

    func test_performNavigation_whenActionPerformerReturnsError_showsGenericErrorWithThatMessage() {
        // Given
        let message = "Error message"
        let navigationError = NavigationError(message: message)
        let navigation = Navigation(action: .navbarLog)
        backendActionPerformer.performBackendReturnValue = .failure(navigationError)
        XCTAssertEqual(subject.genericError.isPresented, false)

        // When
        subject.performFake(navigation: navigation)

        // Then
        XCTAssertEqual(subject.genericError.errorMessage, navigationError.description)
        XCTAssertEqual(subject.genericError.isPresented, true)
    }
}

// MARK: - Mocks

final class BackendNavigationPerformingMock: BackendNavigationPerforming {
    var performBackendActionCallsCount = 0
    var performBackendReceivedAction: [Action] = []
    var performBackendReceivedDetails: [String] = []
    var performBackendReceivedSeedPhrase: [String] = []
    var performBackendReturnValue: Result<ActionResult, NavigationError>!

    var performTransactionActionCallsCount = 0
    var performTransactionReceivedPayload: [String] = []
    var performTransactionReturnValue: Result<ActionResult, TransactionError>!

    func performBackend(action: Action, details: String, seedPhrase: String) -> Result<ActionResult, NavigationError> {
        performBackendActionCallsCount += 1
        performBackendReceivedAction.append(action)
        performBackendReceivedDetails.append(details)
        performBackendReceivedSeedPhrase.append(seedPhrase)
        return performBackendReturnValue
    }

    func performTransaction(with payload: String) -> Result<ActionResult, TransactionError> {
        performTransactionActionCallsCount += 1
        performTransactionReceivedPayload.append(payload)
        return performTransactionReturnValue
    }
}
