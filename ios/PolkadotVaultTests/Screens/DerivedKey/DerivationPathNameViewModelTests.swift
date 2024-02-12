//
//  DerivationPathNameViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 09/02/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import XCTest

final class DerivationPathNameViewModelTests: XCTestCase {
    private var viewModel: DerivationPathNameView.ViewModel!
    private var createKeyServiceMock: CreateDerivedKeyServicingMock!
    private var createKeyNameServiceMock: CreateDerivedKeyNameServicingMock!
    private var seedName: String!
    private var defaultDerivedKeyNameNetworkReturnValue: String!
    private var keySet: MKeysNew!
    private var networkSelection: MmNetwork!
    private var onCompleteCalled: Bool!

    override func setUp() {
        super.setUp()
        onCompleteCalled = false
        defaultDerivedKeyNameNetworkReturnValue = "returnedName"
        keySet = MKeysNew.generate()
        networkSelection = MmNetwork.generate()
        seedName = "seedName"
        createKeyServiceMock = CreateDerivedKeyServicingMock()
        createKeyNameServiceMock = CreateDerivedKeyNameServicingMock()
        createKeyNameServiceMock.defaultDerivedKeyNameNetworkReturnValue = defaultDerivedKeyNameNetworkReturnValue
        viewModel = DerivationPathNameView.ViewModel(
            seedName: seedName,
            keySet: keySet,
            networkSelection: networkSelection,
            createKeyService: createKeyServiceMock,
            createKeyNameService: createKeyNameServiceMock,
            onComplete: {
                self.onCompleteCalled = true
            }
        )
    }

    override func tearDown() {
        viewModel = nil
        createKeyServiceMock = nil
        createKeyNameServiceMock = nil
        super.tearDown()
    }

    func testInitialStateAndPrefillTextField() {
        // Then
        XCTAssertFalse(viewModel.inputText.isEmpty)
        XCTAssertFalse(viewModel.isMainActionDisabled)
    }

    func testInit_createsDefaultDerivedKeyName_withKeySetAndNetworkSelection() {
        // Then
        XCTAssertEqual(createKeyNameServiceMock.defaultDerivedKeyNameNetworkCallsCount, 1)
        XCTAssertEqual(createKeyNameServiceMock.defaultDerivedKeyNameNetworkReceivedKeySet, [keySet])
        XCTAssertEqual(createKeyNameServiceMock.defaultDerivedKeyNameNetworkReceivedNetwork, [networkSelection])
        XCTAssertEqual(createKeyServiceMock.checkForCollisionCompletionCallsCount, 1)
    }

    func testInit_validatesPrefilledName() {
        // Then
        XCTAssertEqual(createKeyServiceMock.checkForCollisionCompletionCallsCount, 1)
        XCTAssertEqual(
            createKeyServiceMock.checkForCollisionCompletionReceivedPath,
            [defaultDerivedKeyNameNetworkReturnValue]
        )
        XCTAssertEqual(createKeyServiceMock.checkForCollisionCompletionReceivedSeedName, [seedName])
    }

    func testOnInfoBoxTap_PresentsInfoModal() {
        // When
        viewModel.onInfoBoxTap()

        // Then
        XCTAssertTrue(viewModel.isPresentingInfoModal)
    }

    func testOnSoftPathTap_AppendsPath() {
        // When
        viewModel.onSoftPathTap()

        // Then
        XCTAssertTrue(viewModel.inputText.contains(DerivationPathComponent.soft.description))
    }

    func testOnHardPathTap_AppendsPath() {
        // When
        viewModel.onHardPathTap()

        // Then
        XCTAssertTrue(viewModel.inputText.contains(DerivationPathComponent.hard.description))
    }

    func testOnPasswordedPathTap_AppendsPath() {
        // When
        viewModel.onPasswordedPathTap()

        // Then
        XCTAssertTrue(viewModel.inputText.contains(DerivationPathComponent.passworded.description))
    }

    func testOnConfirmationCompletion_hidesConfirmation() {
        // When
        viewModel.onConfirmationCompletion()

        // Then
        XCTAssertFalse(viewModel.isPresentingConfirmation)
        XCTAssertTrue(onCompleteCalled)
    }

    func testUnwrappedDerivationPath_whenDerivationPathNil_returnsEmptyString() {
        // Given
        viewModel.derivationPath = nil

        // When
        let result = viewModel.unwrappedDerivationPath()

        // Then
        XCTAssertEqual(result, "")
    }

    func testUnwrappedDerivationPath_whenDerivationPathNotNil_returnsItUnchanged() {
        // Given
        viewModel.derivationPath = "//path"

        // When
        let result = viewModel.unwrappedDerivationPath()

        // Then
        XCTAssertEqual(result, viewModel.derivationPath)
    }

    func testRightNavigationButtonTap_createsDerivedKeyWithExpectedParameters() {
        // Given
        let path = "//path"
        viewModel.inputText = path

        // When
        viewModel.onRightNavigationButtonTap()

        // Then
        XCTAssertEqual(createKeyServiceMock.createDerivedKeyCallsCount, 1)
        XCTAssertEqual(createKeyServiceMock.createDerivedKeyReceivedSeedName, [seedName])
        XCTAssertEqual(createKeyServiceMock.createDerivedKeyReceivedPath, [path])
        XCTAssertEqual(createKeyServiceMock.createDerivedKeyReceivedNetwork, [networkSelection.key])
    }

    func testRightNavigationButtonTap_Success_presentsConfirmation() {
        // Given
        viewModel.inputText = "//path"

        // When
        viewModel.onRightNavigationButtonTap()
        createKeyServiceMock.createDerivedKeyReceivedCompletion.first?(.success(()))

        // Then
        XCTAssertTrue(viewModel.isPresentingConfirmation)
    }

    func testRightNavigationButtonTap_Failure_presentsReturnedError() {
        // Given
        let error = ServiceError(message: "Error")
        let expectedErrorMessage = ErrorBottomModalViewModel.alertError(message: error.localizedDescription)
        viewModel.inputText = "//path"

        // When
        viewModel.onRightNavigationButtonTap()
        createKeyServiceMock.createDerivedKeyReceivedCompletion.first?(.failure(error))

        // Then
        XCTAssertTrue(viewModel.isPresentingError)
        XCTAssertEqual(viewModel.presentableError, expectedErrorMessage)
    }

    func testPasswordConfirmation_whenConfirmationIsCorrect_SetsPasswordAsValid() {
        // Given
        viewModel.isPassworded = true
        viewModel.inputText = "\(DerivationPathComponent.passworded.description)test"
        viewModel.passwordConfirmation = "test"

        // When
        viewModel.onPasswordConfirmationDoneTap()

        // Then
        XCTAssertTrue(viewModel.isPasswordValid)
    }

    func testPasswordConfirmation_whenConfirmationIsNotCorrect_SetsPasswordAsInvalid() {
        // Given
        viewModel.isPassworded = true
        viewModel.inputText = "\(DerivationPathComponent.passworded.description)test"
        viewModel.passwordConfirmation = "tes"

        // When
        viewModel.onPasswordConfirmationDoneTap()

        // Then
        XCTAssertFalse(viewModel.isPasswordValid)
    }

    func testValidateDerivationPath_whenCollision_SetsExpectedError() {
        // Given
        viewModel.inputText = "//path"

        // When
        viewModel.validateDerivationPath()
        createKeyServiceMock.checkForCollisionCompletionReceivedCompletion
            .first?(.success(.generate(collision: .generate())))

        // Then
        XCTAssertEqual(
            viewModel.derivationPathError,
            Localizable.CreateDerivedKey.Modal.Path.Error.alreadyExists.string
        )
    }

    func testValidateDerivationPath_whenButtonNotGood_SetsExpectedError() {
        // Given
        viewModel.inputText = "//path"

        // When
        viewModel.validateDerivationPath()
        createKeyServiceMock.checkForCollisionCompletionReceivedCompletion.first?(.success(.generate(
            buttonGood: false,
            collision: nil
        )))

        // Then
        XCTAssertEqual(viewModel.derivationPathError, Localizable.CreateDerivedKey.Modal.Path.Error.pathInvalid.string)
    }

    func testValidateDerivationPath_whenCollisionWithError_SetsExpectedError() {
        // Given
        let error = "error message"
        viewModel.inputText = "//path"

        // When
        viewModel.validateDerivationPath()
        createKeyServiceMock.checkForCollisionCompletionReceivedCompletion.first?(.success(.generate(
            buttonGood: true,
            collision: nil,
            error: error
        )))

        // Then
        XCTAssertEqual(viewModel.derivationPathError, error)
    }

    func testValidateDerivationPath_whenNoCollision_clearsAnyError() {
        // Given
        viewModel.inputText = "//path"

        // When
        viewModel.validateDerivationPath()
        createKeyServiceMock.checkForCollisionCompletionReceivedCompletion.first?(.success(.generate(
            buttonGood: true,
            collision: nil,
            error: nil
        )))

        // Then
        XCTAssertNil(viewModel.derivationPathError)
    }

    func testValidateDerivationPath_whenServiceError_presentsReturnedError() {
        // Given
        viewModel.inputText = "//path"
        let error = ServiceError(message: "Error")
        let expectedErrorMessage = ErrorBottomModalViewModel.alertError(message: error.localizedDescription)

        // When
        viewModel.validateDerivationPath()
        createKeyServiceMock.checkForCollisionCompletionReceivedCompletion.first?(.failure(error))

        // Then
        XCTAssertTrue(viewModel.isPresentingError)
        XCTAssertEqual(viewModel.presentableError, expectedErrorMessage)
    }
}
