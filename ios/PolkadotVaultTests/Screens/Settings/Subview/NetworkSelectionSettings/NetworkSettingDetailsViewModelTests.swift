//
//  NetworkSettingDetailsViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 12/01/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import XCTest

final class NetworkSettingsDetailsViewModelTests: XCTestCase {
    private var viewModel: NetworkSettingsDetails.ViewModel!
    private var manageNetworkDetailsServiceMock: ManageNetworkDetailsServicingMock!
    private var cancelBag: CancelBag!
    private var onCompleteCalled = false
    private var onCompleteReceivedResult: NetworkSettingsDetails.OnCompletionAction!
    private var networkDetails: MNetworkDetails!

    override func setUp() {
        super.setUp()
        cancelBag = CancelBag()
        manageNetworkDetailsServiceMock = ManageNetworkDetailsServicingMock()
        networkDetails = .generate()
        viewModel = NetworkSettingsDetails.ViewModel(
            networkKey: "testKey",
            networkDetails: networkDetails,
            networkDetailsService: manageNetworkDetailsServiceMock,
            onCompletion: { onCompleteResult in
                self.onCompleteCalled = true
                self.onCompleteReceivedResult = onCompleteResult
            }
        )
    }

    override func tearDown() {
        viewModel = nil
        manageNetworkDetailsServiceMock = nil
        cancelBag = nil
        super.tearDown()
    }

    func testOnAppearUpdatesView() {
        // Given
        let networkDetails = MNetworkDetails.generate()

        // When
        viewModel.onAppear()
        manageNetworkDetailsServiceMock.getNetworkDetailsReceivedCompletion.first?(.success(networkDetails))

        // Then
        XCTAssertEqual(viewModel.networkDetails, networkDetails)
    }

    func testOnAppearWhenFailurePresentsError() {
        // Given
        let error = ServiceError(message: "Error")
        let updatedNetworkDetails = MNetworkDetails.generate(name: "New name")
        let expectedPresentableError: ErrorBottomModalViewModel = .alertError(message: error.localizedDescription)

        // When
        viewModel.onAppear()
        manageNetworkDetailsServiceMock.getNetworkDetailsReceivedCompletion.first?(.failure(error))

        // Then
        XCTAssertNotEqual(viewModel.networkDetails, updatedNetworkDetails)
        XCTAssertTrue(viewModel.isPresentingError)
        XCTAssertEqual(viewModel.presentableError, expectedPresentableError)
    }

    func testRemoveMetadataSuccess() {
        // Given
        let metadataRecord = MMetadataRecord.generate()
        viewModel.didTapDelete(metadataRecord)

        // When
        viewModel.removeMetadata()
        manageNetworkDetailsServiceMock.deleteNetworkMetadataReceivedCompletion.first?(.success(()))

        // Then
        XCTAssertFalse(viewModel.isPresentingRemoveMetadataConfirmation)
        XCTAssertTrue(viewModel.isSnackbarPresented)
    }

    func testRemoveMetadataFailure() {
        // Given
        let error = ServiceError(message: "Error")
        viewModel.didTapDelete(.generate())
        let expectedPresentableError: ErrorBottomModalViewModel = .alertError(message: error.localizedDescription)

        // When
        viewModel.removeMetadata()
        manageNetworkDetailsServiceMock.deleteNetworkMetadataReceivedCompletion.first?(.failure(error))

        // Then
        XCTAssertFalse(viewModel.isPresentingRemoveMetadataConfirmation)
        XCTAssertTrue(viewModel.isPresentingError)
        XCTAssertEqual(viewModel.presentableError, expectedPresentableError)
    }

    func testRemoveNetworkSuccess() {
        // When
        viewModel.removeNetwork()
        manageNetworkDetailsServiceMock.deleteNetworkReceivedCompletion.first?(.success(()))

        // Then
        XCTAssertTrue(onCompleteCalled)
        XCTAssertEqual(onCompleteReceivedResult, .networkDeleted(networkDetails.title))
    }

    func testRemoveNetworkFailure() {
        // Given
        let error = ServiceError(message: "Error")
        let expectedPresentableError: ErrorBottomModalViewModel = .alertError(message: error.localizedDescription)

        // When
        viewModel.removeNetwork()
        manageNetworkDetailsServiceMock.deleteNetworkReceivedCompletion.first?(.failure(error))

        // Then
        XCTAssertTrue(viewModel.isPresentingError)
        XCTAssertEqual(viewModel.presentableError, expectedPresentableError)
    }

    func testOnTapDeleteTriggersConfirmation() {
        // Given
        let metadata = MMetadataRecord.generate()

        // When
        viewModel.didTapDelete(metadata)

        // Then
        XCTAssertTrue(viewModel.isPresentingRemoveMetadataConfirmation)
    }

    func testCancelMetadataRemovalResetsState() {
        // When
        viewModel.cancelMetadataRemoval()

        // Then
        XCTAssertFalse(viewModel.isPresentingRemoveMetadataConfirmation)
    }

    func testOnAddTapOpensQRScanner() {
        // When
        viewModel.onAddTap()

        // Then
        XCTAssertTrue(viewModel.isShowingQRScanner)
    }

    func testOnQRScannerDismissUpdatesView() {
        // When
        viewModel.onQRScannerDismiss()

        // Then
        XCTAssertEqual(manageNetworkDetailsServiceMock.getNetworkDetailsCallsCount, 1)
    }

    func testOnMoreMenuTapOpensActionSheet() {
        // When
        viewModel.onMoreMenuTap()

        // Then
        XCTAssertTrue(viewModel.isShowingActionSheet)
    }

    func testOnMoreActionSheetDismissalWithSignSpecs() {
        // Given
        viewModel.shouldSignSpecs = true

        // When
        viewModel.onMoreActionSheetDismissal()

        // Then
        XCTAssertTrue(viewModel.isPresentingSignSpecList)
        XCTAssertEqual(viewModel.specSignType, .network)
    }

    func testOnMoreActionSheetDismissalWithRemoveNetworkConfirmation() {
        // Given
        viewModel.shouldPresentRemoveNetworkConfirmation = true

        // When
        viewModel.onMoreActionSheetDismissal()

        // Then
        XCTAssertTrue(viewModel.isPresentingRemoveNetworkConfirmation)
        XCTAssertFalse(viewModel.shouldPresentRemoveNetworkConfirmation)
    }

    func testCancelNetworkRemovalResetsState() {
        // Given
        viewModel.isPresentingRemoveNetworkConfirmation = true

        // When
        viewModel.cancelNetworkRemoval()

        // Then
        XCTAssertFalse(viewModel.isPresentingRemoveNetworkConfirmation)
    }

    func testDidTapSignOpensSignSpecList() {
        // Given
        let metadata = MMetadataRecord.generate()

        // When
        viewModel.didTapSign(metadata)

        // Then
        XCTAssertTrue(viewModel.isPresentingSignSpecList)
        XCTAssertEqual(viewModel.specSignType, .metadata(metadataSpecsVersion: metadata.specsVersion))
    }

    func testListenToNavigationUpdatesReflectsView() {
        // Given
        viewModel.isPresentingSignSpecList = true

        // When
        viewModel.isPresentingSignSpecList = false

        // Then
        XCTAssertEqual(manageNetworkDetailsServiceMock.getNetworkDetailsCallsCount, 1)
    }
}
