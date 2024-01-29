//
//  EnterKeySetNameViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 25/01/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import SwiftUI
import XCTest

final class EnterKeySetNameViewModelTests: XCTestCase {
    private var viewModel: EnterKeySetNameView.ViewModel!
    private var seedsMediatorMock: SeedsMediatingMock!
    private var createKeySetServiceMock: CreateKeySetServicingMock!
    private var isPresented: Bool = false
    private var onCompletionActionExecuted: CreateKeysForNetworksView.OnCompletionAction?

    override func setUp() {
        super.setUp()
        seedsMediatorMock = SeedsMediatingMock()
        createKeySetServiceMock = CreateKeySetServicingMock()
        isPresented = false
        viewModel = EnterKeySetNameView.ViewModel(
            seedsMediator: seedsMediatorMock,
            service: createKeySetServiceMock,
            isPresented: Binding(get: { self.isPresented }, set: { self.isPresented = $0 }),
            onCompletion: { action in self.onCompletionActionExecuted = action }
        )
    }

    override func tearDown() {
        viewModel = nil
        seedsMediatorMock = nil
        createKeySetServiceMock = nil
        super.tearDown()
    }

    func testInit_SetsSeedNameToEmpty() {
        // Then
        XCTAssertEqual(viewModel.seedName, "")
    }

    func testOnBackTap_SetsIsPresentedToFalse() {
        // When
        viewModel.onBackTap()

        // Then
        XCTAssertFalse(viewModel.isPresented)
    }

    func testIsActionAvailable_WhenSeedNameIsEmpty_ReturnsFalse() {
        // Given
        viewModel.seedName = ""

        // When
        let result = viewModel.isActionAvailable()

        // Then
        XCTAssertFalse(result)
    }

    func testIsActionAvailable_WhenSeedNameExists_ReturnsFalse() {
        // Given
        viewModel.seedName = "ExistingSeed"
        seedsMediatorMock.checkSeedCollisionSeedNameReturnValue = true

        // When
        let result = viewModel.isActionAvailable()

        // Then
        XCTAssertFalse(result)
    }

    func testIsActionAvailable_WhenSeedNameValid_ReturnsTrue() {
        // Given
        viewModel.seedName = "NewSeed"
        seedsMediatorMock.checkSeedCollisionSeedNameReturnValue = false

        // When
        let result = viewModel.isActionAvailable()

        // Then
        XCTAssertTrue(result)
    }

    func testOnSubmitTap_WhenActionNotAvailable_NoServiceCall() {
        // Given
        viewModel.seedName = ""
        seedsMediatorMock.checkSeedCollisionSeedNameReturnValue = true

        // When
        viewModel.onSubmitTap()

        // Then
        XCTAssertEqual(createKeySetServiceMock.createKeySetSeedNameCallsCount, 0)
    }

    func testOnSubmitTap_WhenActionAvailable_CallsService() {
        // Given
        viewModel.seedName = "NewSeed"
        seedsMediatorMock.checkSeedCollisionSeedNameReturnValue = false

        // When
        viewModel.onSubmitTap()

        // Then
        XCTAssertEqual(createKeySetServiceMock.createKeySetSeedNameCallsCount, 1)
        XCTAssertEqual(createKeySetServiceMock.createKeySetSeedNameReceivedSeedName, [viewModel.seedName])
    }

    func testOnNextTap_ServiceSuccess_SetsDetailsContentAndPresentsDetails() {
        // Given
        let seedBackup = MNewSeedBackup.generate()
        viewModel.seedName = "NewSeed"

        // When
        viewModel.onNextTap()
        createKeySetServiceMock.createKeySetSeedNameReceivedCompletion.first?(.success(seedBackup))

        // Then
        XCTAssertEqual(viewModel.detailsContent, seedBackup)
        XCTAssertTrue(viewModel.isPresentingDetails)
    }

    func testOnNextTap_ServiceFailure_SetsErrorAndPresentsError() {
        // Given
        let error = ServiceError(message: "Error")
        viewModel.seedName = "NewSeed"

        // When
        viewModel.onNextTap()
        createKeySetServiceMock.createKeySetSeedNameReceivedCompletion.first?(.failure(error))

        // Then
        XCTAssertEqual(viewModel.presentableError, .alertError(message: error.localizedDescription))
        XCTAssertTrue(viewModel.isPresentingError)
    }
}
