//
//  CancelBagTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 16/01/2024.
//

import Combine
@testable import PolkadotVault
import XCTest

final class CancelBagTests: XCTestCase {
    func test_cancel_givenSubscriptions_thenSubscriptionsAreCancelled() {
        // Given
        let cancelBag = CancelBag()
        let subscription1 = Just(1).sink { _ in }
        let subscription2 = Just(2).sink { _ in }
        subscription1.store(in: cancelBag)
        subscription2.store(in: cancelBag)

        // When
        cancelBag.cancel()

        // Then
        XCTAssertTrue(cancelBag.subscriptions.isEmpty)
    }

    func test_collect_givenCancellableObjects_thenCancellableObjectsAreAddedToCancelBag() {
        // Given
        let cancelBag = CancelBag()

        // When
        cancelBag.collect {
            Just(1).sink { _ in }
            Just(2).sink { _ in }
        }

        // Then
        XCTAssertEqual(cancelBag.subscriptions.count, 2)
    }

    func test_storeIn_givenCancellableObject_thenCancellableObjectIsStoredInCancelBag() {
        // Given
        let cancelBag = CancelBag()
        let subscription = Just(1).sink { _ in }

        // When
        subscription.store(in: cancelBag)

        // Then
        XCTAssertEqual(cancelBag.subscriptions.count, 1)
    }
}
