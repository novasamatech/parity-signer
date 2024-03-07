//
//  BananaSplitPassphraseModalViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 04/03/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import SwiftUI
import XCTest

final class BananaSplitPassphraseModalViewModelTests: XCTestCase {
    private var viewModel: BananaSplitPassphraseModal.ViewModel!
    private var mediatorMock: KeychainBananaSplitAccessMediatingMock!
    private var isPresented: Bool!
    private var seedName: String!

    override func setUp() {
        super.setUp()
        seedName = "testSeed"
        mediatorMock = KeychainBananaSplitAccessMediatingMock()
        isPresented = true
    }

    override func tearDown() {
        viewModel = nil
        mediatorMock = nil
        seedName = nil
        isPresented = nil
        super.tearDown()
    }

    func testInit_LoadsPassphraseOnSuccess() {
        // Given
        let expectedPassphrase = "loadedPassphrase"
        mediatorMock
            .retrieveBananaSplitPassphraseWithReturnValue =
            .success(BananaSplitPassphrase(passphrase: expectedPassphrase))

        // When
        viewModel = BananaSplitPassphraseModal.ViewModel(
            seedName: seedName,
            isPresented: Binding(get: { self.isPresented }, set: { self.isPresented = $0 }),
            bananaSplitMediator: mediatorMock
        )

        // Then
        XCTAssertEqual(mediatorMock.retrieveBananaSplitPassphraseWithCallsCount, 1)
        XCTAssertEqual(mediatorMock.retrieveBananaSplitPassphraseWithReceivedSeedName, [seedName])
        XCTAssertEqual(viewModel.passphrase, expectedPassphrase)
    }

    func testInit_DoesNotLoadPassphraseOnFailure() {
        // Given
        mediatorMock.retrieveBananaSplitPassphraseWithReturnValue = .failure(.fetchError)

        // When
        viewModel = BananaSplitPassphraseModal.ViewModel(
            seedName: "testSeed",
            isPresented: Binding(get: { self.isPresented }, set: { self.isPresented = $0 }),
            bananaSplitMediator: mediatorMock
        )

        // Then
        XCTAssertEqual(mediatorMock.retrieveBananaSplitPassphraseWithCallsCount, 1)
        XCTAssertEqual(mediatorMock.retrieveBananaSplitPassphraseWithReceivedSeedName, ["testSeed"])
        XCTAssertEqual(viewModel.passphrase, "")
    }

    func testDismissActionSheet_TriggersAnimationAndDismissal() {
        // Given
        let expectation = expectation(description: "AnimateDismissal")
        mediatorMock.retrieveBananaSplitPassphraseWithReturnValue = .failure(.fetchError)
        viewModel = BananaSplitPassphraseModal.ViewModel(
            seedName: seedName,
            isPresented: Binding(get: { self.isPresented }, set: { self.isPresented = $0 }),
            bananaSplitMediator: mediatorMock
        )

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
