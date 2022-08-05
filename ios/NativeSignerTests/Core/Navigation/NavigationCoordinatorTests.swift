//
//  NavigationCoordinatorTests.swift
//  NativeSignerTests
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

@testable import NativeSigner
import XCTest

final class NavigationCoordinatorTests: XCTestCase {
    private var debounceQueue: DispatchingMock!
    private var backendActionPerformer: BackendNavigationPerformingMock!
    private var subject: NavigationCoordinator!

    override func setUp() {
        super.setUp()
        debounceQueue = DispatchingMock()
        backendActionPerformer = BackendNavigationPerformingMock()
        backendActionPerformer.performBackendReturnValue = .generate()
        subject = NavigationCoordinator(
            backendActionPerformer: backendActionPerformer,
            debounceQueue: debounceQueue
        )
    }

    func test_performNavigation_callsBackendPerformerWithExpectedData() {
        // Given
        let expectedAction = Action.goBack
        let expectedDetails = "details"
        let expectedSeedPhrase = "seedPhrase"
        let navigation = Navigation(action: expectedAction, details: expectedDetails, seedPhrase: expectedSeedPhrase)

        // When
        subject.perform(navigation: navigation)

        // Then
        XCTAssertEqual(backendActionPerformer.performBackendActionCallsCount, 1)
        XCTAssertEqual(backendActionPerformer.performBackendReceivedAction, [expectedAction])
        XCTAssertEqual(backendActionPerformer.performBackendReceivedDetails, [expectedDetails])
        XCTAssertEqual(backendActionPerformer.performBackendReceivedSeedPhrase, [expectedSeedPhrase])
    }

    func test_performNavigation_triggersDebounceQueue_withBarrierAndExpectedDetal() {
        // When
        subject.perform(navigation: .init(action: .goBack))

        // Then
        XCTAssertEqual(debounceQueue.asyncAfterCallsCount, 1)
        XCTAssertEqual(debounceQueue.asyncAfterReceivedFlags, [.barrier])
    }

    func test_performNavigation_whenDebounceInProgress_backendCanPerformOnlySingleAction() {
        // Given
        let expectedAction = Action.goBack
        let expectedDetails = "details"
        let expectedSeedPhrase = "seedPhrase"
        let navigation = Navigation(action: expectedAction, details: expectedDetails, seedPhrase: expectedSeedPhrase)
        debounceQueue.shouldPerformAsyncWork = false

        // When
        subject.perform(navigation: navigation)
        subject.perform(navigation: navigation)
        subject.perform(navigation: navigation)
        subject.perform(navigation: navigation)

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
        debounceQueue.shouldPerformAsyncWork = true

        // When
        subject.perform(navigation: firstNavigation)
        subject.perform(navigation: secondNavigation)

        // Then
        XCTAssertEqual(backendActionPerformer.performBackendActionCallsCount, 2)
        XCTAssertEqual(
            backendActionPerformer.performBackendReceivedAction,
            [firstNavigation.action, secondNavigation.action]
        )
        XCTAssertEqual(backendActionPerformer.performBackendReceivedDetails, [firstNavigation.details, ""])
        XCTAssertEqual(backendActionPerformer.performBackendReceivedSeedPhrase, ["", secondNavigation.seedPhrase])
    }
}

// MARK: - Mocks

final class DispatchingMock: Dispatching {
    var shouldPerformAsyncWork = true
    var asyncAfterReceivedFlags: [DispatchWorkItemFlags] = []
    var syncCallsCount = 0
    var asyncCallsCount = 0
    var asyncAfterCallsCount = 0

    func async(execute work: @escaping @convention(block) () -> Void) {
        asyncCallsCount += 1
        guard shouldPerformAsyncWork else { return }
        work()
    }

    func asyncAfter(deadline _: DispatchTime, execute work: @escaping @convention(block) () -> Void) {
        asyncAfterCallsCount += 1
        guard shouldPerformAsyncWork else { return }
        work()
    }

    func asyncAfter(deadline _: DispatchTime, flags: DispatchWorkItemFlags, execute work: @escaping () -> Void) {
        asyncAfterCallsCount += 1
        asyncAfterReceivedFlags.append(flags)
        guard shouldPerformAsyncWork else { return }
        work()
    }

    func sync<T>(flags _: DispatchWorkItemFlags, execute work: () throws -> T) rethrows -> T {
        syncCallsCount += 1
        return try work()
    }

    func sync<T>(execute work: () throws -> T) rethrows -> T {
        syncCallsCount += 1

        return try work()
    }
}

final class BackendNavigationPerformingMock: BackendNavigationPerforming {
    var performBackendActionCallsCount = 0
    var performBackendReceivedAction: [Action] = []
    var performBackendReceivedDetails: [String] = []
    var performBackendReceivedSeedPhrase: [String] = []
    var performBackendReturnValue: ActionResult?

    func performBackend(action: Action, details: String, seedPhrase: String) -> ActionResult? {
        performBackendActionCallsCount += 1
        performBackendReceivedAction.append(action)
        performBackendReceivedDetails.append(details)
        performBackendReceivedSeedPhrase.append(seedPhrase)
        return performBackendReturnValue
    }
}
