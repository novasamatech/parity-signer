//
//  OnboardingMediatorTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 03/01/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import XCTest

final class OnboardingMediatorTests: XCTestCase {
    private var cancellables: Set<AnyCancellable> = []
    private var mediator: OnboardingMediator!
    private var navigationInitialisationService: NavigationInitialisationServicingMock!
    private var seedsMediator: SeedsMediatingMock!
    private var databaseMediator: DatabaseMediatorMock!

    override func setUp() {
        super.setUp()
        navigationInitialisationService = NavigationInitialisationServicingMock()
        seedsMediator = SeedsMediatingMock()
        seedsMediator.removeAllSeedsReturnValue = true
        databaseMediator = DatabaseMediatorMock()
        mediator = OnboardingMediator(
            navigationInitialisationService: navigationInitialisationService,
            seedsMediator: seedsMediator,
            databaseMediator: databaseMediator
        )
    }

    override func tearDown() {
        cancellables = []
        mediator = nil
        navigationInitialisationService = nil
        databaseMediator = nil
        seedsMediator = nil
        super.tearDown()
    }

    func testOnboardingDoneInitialValueReflectsDatabaseAvailabilityWhenTrue() {
        // Given
        let isDatabaseAvailable = true
        databaseMediator.isDatabaseAvailableReturnValue = isDatabaseAvailable
        var receivedValue: Bool?

        // When
        mediator = OnboardingMediator(
            navigationInitialisationService: navigationInitialisationService,
            seedsMediator: seedsMediator,
            databaseMediator: databaseMediator
        )
        mediator.onboardingDone
            .sink { receivedValue = $0 }
            .store(in: &cancellables)

        // Then
        XCTAssertEqual(
            receivedValue,
            isDatabaseAvailable
        )
    }

    func testOnboardingDoneInitialValueReflectsDatabaseAvailabilityWhenFalse() {
        // Given
        let isDatabaseAvailable = false
        databaseMediator.isDatabaseAvailableReturnValue = isDatabaseAvailable
        var receivedValue: Bool?

        // When
        mediator = OnboardingMediator(
            navigationInitialisationService: navigationInitialisationService,
            seedsMediator: seedsMediator,
            databaseMediator: databaseMediator
        )
        mediator.onboardingDone
            .sink { receivedValue = $0 }
            .store(in: &cancellables)

        // Then
        XCTAssertEqual(
            receivedValue,
            isDatabaseAvailable
        )
    }

    func testOnboardingWhenRemoveAllSeedsDoneCallsInitialisationNavigationWithPassedValue() {
        // Given
        let expectedValue = true
        seedsMediator.removeAllSeedsReturnValue = true

        // When
        mediator.onboard(verifierRemoved: expectedValue)

        // Then
        XCTAssertEqual(navigationInitialisationService.initialiseNavigationVerifierRemovedCallsCount, 1)
        XCTAssertEqual(
            navigationInitialisationService.initialiseNavigationVerifierRemovedReceivedVerifierRemoved,
            [expectedValue]
        )
    }

    func testOnboardingWhenRemoveAllSeedsDoneRecreatesDatabaseFiles() {
        // Given
        seedsMediator.removeAllSeedsReturnValue = true

        // When
        mediator.onboard(verifierRemoved: true)

        // Then
        XCTAssertEqual(databaseMediator.recreateDatabaseFileCallsCount, 1)
    }

    func testOnboardingFailsToUpdateOnboardingDoneWhenSeedsRemovalFails() {
        // Given
        let isDatabaseAvailable = false
        databaseMediator.isDatabaseAvailableReturnValue = isDatabaseAvailable
        seedsMediator.removeAllSeedsReturnValue = false
        var receivedValue: Bool?
        let expectation = XCTestExpectation()

        // When
        mediator.onboard(verifierRemoved: false)

        // Then
        mediator.onboardingDone
            .sink { value in
                receivedValue = value
                expectation.fulfill()
            }
            .store(in: &cancellables)
        wait(for: [expectation], timeout: 1.0)
        XCTAssertEqual(receivedValue, isDatabaseAvailable)
    }

    func testOnboardingUpdatesOnboardingDoneToTrue() {
        // Given
        seedsMediator.removeAllSeedsReturnValue = true
        var receivedValue: Bool?
        let expectation = XCTestExpectation()

        // When
        mediator.onboard(verifierRemoved: true)
        navigationInitialisationService.initialiseNavigationVerifierRemovedReceivedCompletion.first?(.success(()))

        // Then
        mediator.onboardingDone
            .sink { value in
                receivedValue = value
                expectation.fulfill()
            }
            .store(in: &cancellables)
        wait(for: [expectation], timeout: 1.0)
        XCTAssertTrue(receivedValue ?? false)
    }
}
