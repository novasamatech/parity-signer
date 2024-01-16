//
//  SignSpecsListViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 15/01/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import XCTest

final class SignSpecsListViewViewModelTests: XCTestCase {
    private var viewModel: SignSpecsListView.ViewModel!
    private var seedsMediatorMock: SeedsMediatingMock!
    private var manageNetworkDetailsServiceMock: ManageNetworkDetailsServicingMock!
    private var cancellables: Set<AnyCancellable>!

    override func setUp() {
        super.setUp()
        seedsMediatorMock = SeedsMediatingMock()
        manageNetworkDetailsServiceMock = ManageNetworkDetailsServicingMock()
        viewModel = SignSpecsListView.ViewModel(
            networkKey: "networkKey",
            type: .network,
            seedsMediator: seedsMediatorMock,
            service: manageNetworkDetailsServiceMock
        )
        cancellables = []
    }

    override func tearDown() {
        viewModel = nil
        seedsMediatorMock = nil
        manageNetworkDetailsServiceMock = nil
        cancellables = nil
        super.tearDown()
    }

    func testViewModelOnAppearLoadsData() {
        // Given
        let expectedContent = MSignSufficientCrypto.generate()

        // When
        viewModel.onAppear()
        manageNetworkDetailsServiceMock.signSpecListReceivedCompletion.first?(.success(expectedContent))

        // Then
        XCTAssertEqual(manageNetworkDetailsServiceMock.signSpecListCallsCount, 1)
        XCTAssertEqual(viewModel.content, expectedContent)
    }

    func testPresentErrorOnFailedOnAppear() {
        // Given
        let error: ServiceError = .init(message: "Error")
        let expectedPresentableError: ErrorBottomModalViewModel = .alertError(message: error.localizedDescription)

        // When
        viewModel.onAppear()
        manageNetworkDetailsServiceMock.signSpecListReceivedCompletion.first?(.failure(error))

        // Then
        XCTAssertTrue(viewModel.isPresentingError)
        XCTAssertEqual(viewModel.presentableError, expectedPresentableError)
    }

    func testViewModelOnRecordTapWithPassword() {
        // Given
        let keyRecord = MRawKey.generate(address: .generate(hasPwd: true))
        seedsMediatorMock.getSeedSeedNameReturnValue = "passwordProtectedSeed"

        // When
        viewModel.onRecordTap(keyRecord)

        // Then
        XCTAssertEqual(viewModel.selectedKeyRecord, keyRecord)
        XCTAssertTrue(viewModel.isPresentingEnterPassword)
    }

    func testOnRecordTap_AttemptSigningWithoutPassword() {
        // Given
        let keyRecordWithoutPassword = MRawKey.generate()
        seedsMediatorMock.getSeedSeedNameReturnValue = "SeedPhrase"

        // When
        viewModel.onRecordTap(keyRecordWithoutPassword)
        manageNetworkDetailsServiceMock.signSpecSigningAddressKeySeedPhrasePasswordReceivedCompletion
            .first?(.success(MSufficientCryptoReady.generate()))

        // Then
        XCTAssertEqual(seedsMediatorMock.getSeedSeedNameCallsCount, 1)
        XCTAssertEqual(seedsMediatorMock.getSeedSeedNameReceivedSeedName, [keyRecordWithoutPassword.address.seedName])
        XCTAssertEqual(manageNetworkDetailsServiceMock.signSpecSigningAddressKeySeedPhrasePasswordCallsCount, 1)
        XCTAssertTrue(viewModel.isPresentingDetails)
        XCTAssertNotNil(viewModel.detailsContent)
    }

    func testOnRecordTap_AttemptSigningWithoutPassword_whenError_presentsError() {
        // Given
        let error: ServiceError = .init(message: "Error")
        let expectedPresentableError: ErrorBottomModalViewModel = .alertError(message: error.localizedDescription)
        let keyRecordWithoutPassword = MRawKey.generate()
        seedsMediatorMock.getSeedSeedNameReturnValue = "SeedPhrase"

        // When
        viewModel.onRecordTap(keyRecordWithoutPassword)
        manageNetworkDetailsServiceMock.signSpecSigningAddressKeySeedPhrasePasswordReceivedCompletion
            .first?(.failure(.error(error)))

        // Then
        XCTAssertTrue(viewModel.isPresentingError)
        XCTAssertEqual(viewModel.presentableError, expectedPresentableError)
    }

    func testViewModelOnPasswordModalDoneWithSuccess() {
        // Given
        let keyRecord = MRawKey.generate()
        let detailsContent = MSufficientCryptoReady.generate()
        let modalViewModel = SignSpecEnterPasswordModal.ViewModel(
            isPresented: .constant(true),
            selectedKeyRecord: keyRecord,
            onDoneTapAction: { _ in }
        )
        seedsMediatorMock.getSeedSeedNameReturnValue = "seedPhrase"

        // When
        viewModel.onPasswordModalDoneTapAction(modalViewModel)
        manageNetworkDetailsServiceMock.signSpecSigningAddressKeySeedPhrasePasswordReceivedCompletion
            .first?(.success(detailsContent))

        // Then
        XCTAssertEqual(manageNetworkDetailsServiceMock.signSpecSigningAddressKeySeedPhrasePasswordCallsCount, 1)
        XCTAssertEqual(viewModel.detailsContent, detailsContent)
        XCTAssertTrue(viewModel.isPresentingDetails)
    }

    func testViewModelOnPasswordModalDoneWithError() {
        // Given
        let keyRecord = MRawKey.generate()
        let modalViewModel = SignSpecEnterPasswordModal.ViewModel(
            isPresented: .constant(true),
            selectedKeyRecord: keyRecord,
            onDoneTapAction: { _ in }
        )
        let error: SpecSignError = .error(.init(message: "Error"))

        seedsMediatorMock.getSeedSeedNameReturnValue = "seedPhrase"

        // When
        viewModel.onPasswordModalDoneTapAction(modalViewModel)
        manageNetworkDetailsServiceMock.signSpecSigningAddressKeySeedPhrasePasswordReceivedCompletion
            .first?(.failure(error))

        // Then
        XCTAssertEqual(manageNetworkDetailsServiceMock.signSpecSigningAddressKeySeedPhrasePasswordCallsCount, 1)
        XCTAssertFalse(viewModel.isPresentingDetails)
        XCTAssertTrue(viewModel.isPresentingError)
    }

    func testAttemptSigningWithEmptySeedPhrase() {
        // Given
        let keyRecord = MRawKey.generate()
        seedsMediatorMock.getSeedSeedNameReturnValue = ""

        // When
        viewModel.onRecordTap(keyRecord)

        // Then
        XCTAssertEqual(seedsMediatorMock.getSeedSeedNameCallsCount, 1)
        XCTAssertNil(viewModel.detailsContent)
        XCTAssertFalse(viewModel.isPresentingDetails)
    }

    func testAttemptSigningWithValidSeedPhrase() {
        // Given
        let keyRecord = MRawKey.generate()
        seedsMediatorMock.getSeedSeedNameReturnValue = ""

        // When
        viewModel.onRecordTap(keyRecord)

        // Then
        XCTAssertEqual(seedsMediatorMock.getSeedSeedNameCallsCount, 1)
        XCTAssertNil(viewModel.detailsContent)
        XCTAssertFalse(viewModel.isPresentingDetails)
    }

    func testOnPasswordModalDoneTapAction_WrongPasswordError() {
        // Given
        let expectedKeyRecord = MRawKey.generate()
        let modalViewModel = SignSpecEnterPasswordModal.ViewModel(
            isPresented: .constant(true),
            selectedKeyRecord: expectedKeyRecord,
            onDoneTapAction: { _ in }
        )
        viewModel.selectedKeyRecord = expectedKeyRecord
        seedsMediatorMock.getSeedSeedNameReturnValue = "SomeSeedPhrase"

        // When
        viewModel.onPasswordModalDoneTapAction(modalViewModel)
        manageNetworkDetailsServiceMock.signSpecSigningAddressKeySeedPhrasePasswordReceivedCompletion
            .first?(.failure(.wrongPassword))

        // Then
        XCTAssertFalse(modalViewModel.isValid)
    }
}
