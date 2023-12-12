//
//  KeyDetailsPublicKeyViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 11/12/2023.
//

import Foundation
@testable import PolkadotVault
import XCTest

final class KeyDetailsPublicKeyViewRenderableTests: XCTestCase {
    private var publicKeyDetailsServiceMock: PublicKeyDetailsServicingMock!
    private var exportPrivateKeyServiceMock: ExportPrivateKeyServicingMock!
    private var keyDetailsServiceMock: KeyDetailsActionServiceMock!
    private var keyDetails: MKeyDetails!
    private var viewModel: KeyDetailsPublicKeyView.ViewModel!

    override func setUp() {
        super.setUp()
        publicKeyDetailsServiceMock = PublicKeyDetailsServicingMock()
        exportPrivateKeyServiceMock = ExportPrivateKeyServicingMock()
        keyDetailsServiceMock = KeyDetailsActionServiceMock()
        keyDetails = MKeyDetails.generate()
        viewModel = KeyDetailsPublicKeyView.ViewModel(
            keyDetails: keyDetails,
            addressKey: "mockAddressKey",
            publicKeyDetailsService: publicKeyDetailsServiceMock,
            exportPrivateKeyService: exportPrivateKeyServiceMock,
            keyDetailsService: keyDetailsServiceMock,
            onCompletion: { _ in }
        )
    }

    override func tearDown() {
        publicKeyDetailsServiceMock = nil
        exportPrivateKeyServiceMock = nil
        keyDetailsServiceMock = nil
        viewModel = nil
        super.tearDown()
    }

    func testIsExportKeyAvailable_NoPassword() {
        // Given
        let keyDetailsWithPassword = MKeyDetails.generate(address: .generate(hasPwd: true))

        // When
        viewModel = KeyDetailsPublicKeyView.ViewModel(
            keyDetails: keyDetailsWithPassword,
            addressKey: "mockAddressKey",
            publicKeyDetailsService: publicKeyDetailsServiceMock,
            onCompletion: { _ in }
        )

        // Then
        XCTAssertFalse(viewModel.isExportKeyAvailable)
    }

    func testIsExportKeyAvailable_Password() {
        // Given
        let keyDetailsWithoutPassword = MKeyDetails.generate(address: .generate(hasPwd: false))

        // When
        viewModel = KeyDetailsPublicKeyView.ViewModel(
            keyDetails: keyDetailsWithoutPassword,
            addressKey: "mockAddressKey",
            publicKeyDetailsService: publicKeyDetailsServiceMock,
            onCompletion: { _ in }
        )

        // Then
        XCTAssertTrue(viewModel.isExportKeyAvailable)
    }

    func testOnMoreButtonTap() {
        // When
        viewModel.onMoreButtonTap()

        // Then
        XCTAssertTrue(viewModel.isShowingActionSheet)
    }

    func testCheckForActionsPresentation_ExportPrivateKey_Success() {
        // Given
        viewModel.shouldPresentExportKeysWarningModal = true
        let expectedModel = ExportPrivateKeyViewModel.generate()

        // When
        viewModel.checkForActionsPresentation()
        exportPrivateKeyServiceMock.exportPrivateKeyCompletionReceivedCompletion.first?(.success(expectedModel))

        // Then
        XCTAssertEqual(exportPrivateKeyServiceMock.exportPrivateKeyCompletionCallsCount, 1)
        XCTAssertFalse(viewModel.shouldPresentExportKeysWarningModal)
        XCTAssertEqual(viewModel.exportPrivateKeyViewModel, expectedModel)
        XCTAssertTrue(viewModel.isPresentingExportKeysWarningModal)
    }

    func testCheckForActionsPresentation_ExportPrivateKey_Failure() {
        // Given
        viewModel.shouldPresentExportKeysWarningModal = true
        let error = ServiceError(message: "Error occurred")

        // When
        viewModel.checkForActionsPresentation()
        exportPrivateKeyServiceMock.exportPrivateKeyCompletionReceivedCompletion.first?(.failure(error))

        // Then
        XCTAssertEqual(exportPrivateKeyServiceMock.exportPrivateKeyCompletionCallsCount, 1)
        XCTAssertFalse(viewModel.shouldPresentExportKeysWarningModal)
        XCTAssertEqual(viewModel.presentableError, .alertError(message: error.message))
        XCTAssertTrue(viewModel.isPresentingError)
    }

    func testCheckForActionsPresentation_RemoveConfirmationModal() {
        // Given
        viewModel.shouldPresentRemoveConfirmationModal = true

        // When
        viewModel.checkForActionsPresentation()

        // Then
        XCTAssertFalse(viewModel.shouldPresentRemoveConfirmationModal)
        XCTAssertTrue(viewModel.isShowingRemoveConfirmation)
    }

    func testOnWarningDismissal() {
        // Given
        viewModel.shouldPresentExportKeysModal = true

        // When
        viewModel.onWarningDismissal()

        // Then
        XCTAssertFalse(viewModel.shouldPresentExportKeysModal)
        XCTAssertTrue(viewModel.isPresentingExportKeysModal)
    }

    func testOnExportKeysDismissal_Success() {
        // Given
        let mockKeyDetails = MKeyDetails.generate()

        // When
        viewModel.onExportKeysDismissal()
        keyDetailsServiceMock.publicKeyCompletions.first?(.success(mockKeyDetails))

        // Then
        XCTAssertEqual(keyDetailsServiceMock.publicKeyCallsCount, 1)
        XCTAssertEqual(viewModel.keyDetails, mockKeyDetails)
        XCTAssertNil(viewModel.exportPrivateKeyViewModel)
    }

    func testOnExportKeysDismissal_Failure() {
        // Given
        let error = ServiceError(message: "Error occurred")

        // When
        viewModel.onExportKeysDismissal()
        keyDetailsServiceMock.publicKeyCompletions.first?(.failure(error))

        // Then
        XCTAssertEqual(keyDetailsServiceMock.publicKeyCallsCount, 1)
        XCTAssertEqual(viewModel.presentableError, .alertError(message: error.localizedDescription))
        XCTAssertTrue(viewModel.isPresentingError)
        XCTAssertNil(viewModel.exportPrivateKeyViewModel)
    }

    func testOnRemoveKeyTap_Success() {
        // Given
        var onCompletionCalled = false
        var completionAction: KeyDetailsPublicKeyView.OnCompletionAction?
        viewModel = KeyDetailsPublicKeyView.ViewModel(
            keyDetails: MKeyDetails.generate(),
            addressKey: "mockAddressKey",
            publicKeyDetailsService: publicKeyDetailsServiceMock,
            onCompletion: { action in
                onCompletionCalled = true
                completionAction = action
            }
        )

        // When
        viewModel.onRemoveKeyTap()
        publicKeyDetailsServiceMock.forgetSingleKeyAddressNetworkSpecsKeyReceivedCompletion.first?(.success(()))

        // Then
        XCTAssertEqual(publicKeyDetailsServiceMock.forgetSingleKeyAddressNetworkSpecsKeyCallsCount, 1)
        XCTAssertTrue(onCompletionCalled)
        XCTAssertEqual(completionAction, .derivedKeyDeleted)
    }

    func testOnRemoveKeyTap_Failure() {
        // Given
        let error = ServiceError(message: "Error occurred")

        // When
        viewModel.onRemoveKeyTap()
        publicKeyDetailsServiceMock.forgetSingleKeyAddressNetworkSpecsKeyReceivedCompletion.first?(.failure(error))

        // Then
        XCTAssertEqual(publicKeyDetailsServiceMock.forgetSingleKeyAddressNetworkSpecsKeyCallsCount, 1)
        XCTAssertEqual(viewModel.presentableError, .alertError(message: error.localizedDescription))
        XCTAssertTrue(viewModel.isPresentingError)
    }
}

// MARK: - Mocks

final class PublicKeyDetailsServiceMock: PublicKeyDetailsServicing {
    var forgetSingleKeyCallsCount = 0
    var forgetSingleKeyReceivedArguments: [(address: String, networkSpecsKey: String)] = []
    var forgetSingleKeyCompletion: ((Result<Void, ServiceError>) -> Void)?

    func forgetSingleKey(
        address: String,
        networkSpecsKey: String,
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    ) {
        forgetSingleKeyCallsCount += 1
        forgetSingleKeyReceivedArguments.append((address, networkSpecsKey))
        forgetSingleKeyCompletion = completion
    }
}

final class KeyDetailsActionServiceMock: KeyDetailsActionServicing {
    var performBackupSeedCallsCount = 0
    var performBackupSeedReceivedSeedNames: [String] = []
    var performBackupSeedCompletions: [(Result<Void, ServiceError>) -> Void] = []

    var publicKeyCallsCount = 0
    var publicKeyReceivedArguments: [(addressKey: String, networkSpecsKey: String)] = []
    var publicKeyCompletions: [(Result<MKeyDetails, ServiceError>) -> Void] = []

    var forgetKeySetCallsCount = 0
    var forgetKeySetReceivedSeedNames: [String] = []
    var forgetKeySetCompletions: [(Result<Void, ServiceError>) -> Void] = []

    func performBackupSeed(
        seedName: String,
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    ) {
        performBackupSeedCallsCount += 1
        performBackupSeedReceivedSeedNames.append(seedName)
        performBackupSeedCompletions.append(completion)
    }

    func publicKey(
        addressKey: String,
        networkSpecsKey: String,
        _ completion: @escaping (Result<MKeyDetails, ServiceError>) -> Void
    ) {
        publicKeyCallsCount += 1
        publicKeyReceivedArguments.append((addressKey, networkSpecsKey))
        publicKeyCompletions.append(completion)
    }

    func forgetKeySet(
        seedName: String,
        _ completion: @escaping (Result<Void, ServiceError>) -> Void
    ) {
        forgetKeySetCallsCount += 1
        forgetKeySetReceivedSeedNames.append(seedName)
        forgetKeySetCompletions.append(completion)
    }
}
