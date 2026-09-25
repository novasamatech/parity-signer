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

        XCTAssertEqual(mediator.isUserOnboarded, isDatabaseAvailable)
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

    func testPrepareForScanningRemovesSeedsRecreatesDatabaseAndInitialisesNavigationWithCertificate() {
        databaseMediator.recreateDatabaseFileReturnValue = true

        mediator.prepareForScanning { _ in }

        XCTAssertEqual(seedsMediator.removeAllSeedsCallsCount, 1)
        XCTAssertEqual(databaseMediator.recreateDatabaseFileCallsCount, 1)
        XCTAssertEqual(navigationInitialisationService.initialiseNavigationVerifierRemovedCallsCount, 1)
        XCTAssertEqual(
            navigationInitialisationService.initialiseNavigationVerifierRemovedReceivedVerifierRemoved,
            [false]
        )
    }

    func testPrepareForScanningWhenNavigationInitialisedCompletesWithoutFinishingOnboarding() {
        databaseMediator.recreateDatabaseFileReturnValue = true
        var isPrepared: Bool?
        var onboardingDone: Bool?
        mediator.onboardingDone
            .sink { onboardingDone = $0 }
            .store(in: &cancellables)

        mediator.prepareForScanning { isPrepared = $0 }
        navigationInitialisationService.initialiseNavigationVerifierRemovedReceivedCompletion.first?(.success(()))

        XCTAssertEqual(isPrepared, true)
        XCTAssertEqual(onboardingDone, false)
        XCTAssertEqual(seedsMediator.refreshSeedsCallsCount, 0)
    }

    func testPrepareForScanningWhenNavigationInitialisationFailsCompletesWithFailure() {
        databaseMediator.recreateDatabaseFileReturnValue = true
        var isPrepared: Bool?

        mediator.prepareForScanning { isPrepared = $0 }
        navigationInitialisationService.initialiseNavigationVerifierRemovedReceivedCompletion
            .first?(.failure(.init(message: "")))

        XCTAssertEqual(isPrepared, false)
    }

    func testPrepareForScanningWhenSeedsRemovalFailsDoesNotRecreateDatabase() {
        seedsMediator.removeAllSeedsReturnValue = false
        var isPrepared: Bool?

        mediator.prepareForScanning { isPrepared = $0 }

        XCTAssertEqual(isPrepared, false)
        XCTAssertEqual(databaseMediator.recreateDatabaseFileCallsCount, 0)
        XCTAssertEqual(navigationInitialisationService.initialiseNavigationVerifierRemovedCallsCount, 0)
    }

    func testPrepareForScanningWhenDatabaseRecreationFailsDoesNotInitialiseNavigation() {
        databaseMediator.recreateDatabaseFileReturnValue = false
        var isPrepared: Bool?

        mediator.prepareForScanning { isPrepared = $0 }

        XCTAssertEqual(isPrepared, false)
        XCTAssertEqual(navigationInitialisationService.initialiseNavigationVerifierRemovedCallsCount, 0)
    }

    func testFinishOnboardingUpdatesOnboardingDoneToTrueWithoutRecreatingDatabase() {
        var onboardingDone: Bool?
        mediator.onboardingDone
            .sink { onboardingDone = $0 }
            .store(in: &cancellables)

        mediator.finishOnboarding()

        XCTAssertEqual(onboardingDone, true)
        XCTAssertEqual(seedsMediator.refreshSeedsCallsCount, 1)
        XCTAssertEqual(seedsMediator.removeAllSeedsCallsCount, 0)
        XCTAssertEqual(databaseMediator.recreateDatabaseFileCallsCount, 0)
        XCTAssertEqual(navigationInitialisationService.initialiseNavigationVerifierRemovedCallsCount, 0)
    }
}
