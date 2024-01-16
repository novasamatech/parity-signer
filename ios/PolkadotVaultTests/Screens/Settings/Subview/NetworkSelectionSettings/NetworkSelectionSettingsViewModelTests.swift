//
//  NetworkSelectionSettingsViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 11/01/2024.
//

import Combine
@testable import PolkadotVault
import XCTest

final class NetworkSelectionSettingsViewModelTests: XCTestCase {
    private var viewModel: NetworkSelectionSettings.ViewModel!
    private var getManagedNetworksServiceMock: GetManagedNetworksServicingMock!
    private var manageNetworkDetailsServiceMock: ManageNetworkDetailsServicingMock!

    override func setUp() {
        super.setUp()
        getManagedNetworksServiceMock = GetManagedNetworksServicingMock()
        manageNetworkDetailsServiceMock = ManageNetworkDetailsServicingMock()
        viewModel = NetworkSelectionSettings.ViewModel(
            service: getManagedNetworksServiceMock,
            networkDetailsService: manageNetworkDetailsServiceMock
        )
    }

    override func tearDown() {
        viewModel = nil
        getManagedNetworksServiceMock = nil
        manageNetworkDetailsServiceMock = nil
        super.tearDown()
    }

    func testOnTapNetwork_FetchesAndPresentsDetailsOnSuccess() {
        // Given
        let networkKey = "networkKey"
        let mockNetwork = MmNetwork.generate(key: networkKey)
        let mockDetails = MNetworkDetails.generate()

        // When
        viewModel.onTap(mockNetwork)
        manageNetworkDetailsServiceMock.getNetworkDetailsReceivedCompletion.first?(.success(mockDetails))

        // Then
        XCTAssertEqual(manageNetworkDetailsServiceMock.getNetworkDetailsCallsCount, 1)
        XCTAssertEqual(viewModel.selectedDetails, mockDetails)
        XCTAssertTrue(viewModel.isPresentingDetails)
    }

    func testOnTapNetwork_PresentsErrorOnFailure() {
        // Given
        let networkKey = "networkKey"
        let mockNetwork = MmNetwork.generate(key: networkKey)
        let mockError = ServiceError(message: "Error occurred")
        let expectedPresentableError: ErrorBottomModalViewModel = .alertError(message: mockError.localizedDescription)

        // When
        viewModel.onTap(mockNetwork)
        manageNetworkDetailsServiceMock.getNetworkDetailsReceivedCompletion.first?(.failure(mockError))

        // Then
        XCTAssertEqual(manageNetworkDetailsServiceMock.getNetworkDetailsCallsCount, 1)
        XCTAssertEqual(viewModel.presentableError, expectedPresentableError)
        XCTAssertTrue(viewModel.isPresentingError)
    }

    func testOnInit_SuccessfulNetworkFetch() {
        // Given
        let mockNetworks = [MmNetwork.generate(key: "key1"), MmNetwork.generate(key: "key2")]

        // When
        getManagedNetworksServiceMock.getNetworksReceivedCompletion.first?(.success(mockNetworks))

        // Then
        XCTAssertEqual(getManagedNetworksServiceMock.getNetworksCallsCount, 1)
        XCTAssertEqual(viewModel.networks, mockNetworks)
    }

    func testOnInit_FailureInNetworkFetch() {
        // Given
        let mockError = ServiceError(message: "Error occurred")
        let expectedPresentableError: ErrorBottomModalViewModel = .alertError(message: mockError.localizedDescription)

        // When
        getManagedNetworksServiceMock.getNetworksReceivedCompletion.first?(.failure(mockError))

        // Then
        XCTAssertEqual(getManagedNetworksServiceMock.getNetworksCallsCount, 1)
        XCTAssertEqual(viewModel.presentableError, expectedPresentableError)
        XCTAssertTrue(viewModel.isPresentingError)
    }

    func testOnQRScannerDismiss_UpdatesNetworks() {
        // Given
        let mockNetworks = [MmNetwork.generate(key: "key1"), MmNetwork.generate(key: "key2")]
        let updatedMockNetworks = [MmNetwork.generate(key: "updatedKey")]
        getManagedNetworksServiceMock.getNetworksReceivedCompletion.first?(.success(mockNetworks))

        // When
        viewModel.onQRScannerDismiss()
        getManagedNetworksServiceMock.getNetworksReceivedCompletion.last?(.success(updatedMockNetworks))

        // Then
        XCTAssertEqual(viewModel.networks, updatedMockNetworks)
        XCTAssertEqual(getManagedNetworksServiceMock.getNetworksCallsCount, 2)
    }

    func testOnNetworkDetailsCompletion_DisplaysSnackbarOnNetworkDeleted() {
        // Given
        let networkTitle = "Test Network"

        // When
        viewModel.onNetworkDetailsCompletion(.networkDeleted(networkTitle))

        // Then
        XCTAssertEqual(
            viewModel.snackbarViewModel.title,
            Localizable.Settings.NetworkDetails.DeleteNetwork.Label.confirmation(networkTitle)
        )
        XCTAssertTrue(viewModel.isSnackbarPresented)
    }

    func testOnIsPresentingDetailsChange_UpdatesNetworks() {
        // Given
        let mockNetworks = [MmNetwork.generate(key: "key1"), MmNetwork.generate(key: "key2")]
        let updatedMockNetworks = [MmNetwork.generate(key: "updatedKey")]
        getManagedNetworksServiceMock.getNetworksReceivedCompletion.first?(.success(mockNetworks))
        viewModel.isPresentingDetails = true

        // When
        viewModel.isPresentingDetails = false
        getManagedNetworksServiceMock.getNetworksReceivedCompletion.last?(.success(updatedMockNetworks))

        // Then
        XCTAssertEqual(viewModel.networks, updatedMockNetworks)
        XCTAssertEqual(getManagedNetworksServiceMock.getNetworksCallsCount, 2)
    }
}
