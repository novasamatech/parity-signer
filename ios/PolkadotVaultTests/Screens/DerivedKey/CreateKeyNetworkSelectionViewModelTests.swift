//
//  CreateKeyNetworkSelectionViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 12/02/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import XCTest

final class CreateKeyNetworkSelectionViewModelTests: XCTestCase {
    private var viewModel: CreateKeyNetworkSelectionView.ViewModel!
    private var networkServiceMock: GetManagedNetworksServicingMock!
    private var createKeyServiceMock: CreateDerivedKeyServicingMock!
    private var onCompletionAction: CreateKeyNetworkSelectionView.OnCompletionAction?
    private var seedName: String!
    private var keyName: String!
    private var keySet: MKeysNew!

    override func setUp() {
        super.setUp()
        keySet = .generate()
        seedName = "seedName"
        keyName = "keyName"
        networkServiceMock = GetManagedNetworksServicingMock()
        createKeyServiceMock = CreateDerivedKeyServicingMock()
        viewModel = CreateKeyNetworkSelectionView.ViewModel(
            seedName: seedName,
            keyName: keyName,
            keySet: keySet,
            networkService: networkServiceMock,
            createKeyService: createKeyServiceMock,
            onCompletion: { action in
                self.onCompletionAction = action
            }
        )
    }

    override func tearDown() {
        viewModel = nil
        networkServiceMock = nil
        createKeyServiceMock = nil
        onCompletionAction = nil
        super.tearDown()
    }

    func testLoadNetworks_OnInit_WhenSuccess_loadsNetworksFromService() {
        // Given
        let expectedNetworks = [MmNetwork.generate(), MmNetwork.generate(title: "Network 2")]

        // When
        networkServiceMock.getNetworksReceivedCompletion.first?(.success(expectedNetworks))

        // Then
        XCTAssertEqual(viewModel.networks.count, expectedNetworks.count)
        XCTAssertEqual(networkServiceMock.getNetworksCallsCount, 1)
    }

    func testSelectNetwork_ChangesSelection_toNetwork() {
        // Given
        let network = MmNetwork.generate()
        viewModel.networks.append(network)

        // When
        viewModel.selectNetwork(network)

        // Then
        XCTAssertEqual(viewModel.networkSelection, network)
    }

    func testSelectNetwork_whenNetworkAlreadySelected_ChangesSelectionToNone() {
        // Given
        let network = MmNetwork.generate()
        viewModel.networkSelection = network

        // When
        viewModel.selectNetwork(network)

        // Then
        XCTAssertNil(viewModel.networkSelection)
    }

    func testDidTapCreate_WithoutSelection_NoAction() {
        // When
        viewModel.didTapCreate()

        // Then
        XCTAssertEqual(createKeyServiceMock.createDefaultDerivedKeyCompletionCallsCount, 0)
    }

    func testDidTapCreate_WithSelection_Success() {
        // Given
        let network = MmNetwork.generate()
        viewModel.networks = [network]
        viewModel.selectNetwork(network)

        // When
        viewModel.didTapCreate()
        createKeyServiceMock.createDefaultDerivedKeyCompletionReceivedCompletion.first?(.success(()))

        // Then
        XCTAssertEqual(createKeyServiceMock.createDefaultDerivedKeyCompletionCallsCount, 1)
        XCTAssertEqual(createKeyServiceMock.createDefaultDerivedKeyCompletionReceivedNetwork, [network])
        XCTAssertEqual(createKeyServiceMock.createDefaultDerivedKeyCompletionReceivedKeySet, [keySet])
        XCTAssertEqual(createKeyServiceMock.createDefaultDerivedKeyCompletionReceivedKeyName, [keyName])
        XCTAssertNotNil(onCompletionAction)
    }

    func testDidTapCreate_WithSelection_Failure() {
        // Given
        let network = MmNetwork.generate()
        viewModel.networks = [network]
        viewModel.selectNetwork(network)
        let error = ServiceError(message: "Error")
        let expectedErrorMessage = ErrorBottomModalViewModel.alertError(message: error.localizedDescription)

        // When
        viewModel.didTapCreate()
        createKeyServiceMock.createDefaultDerivedKeyCompletionReceivedCompletion.first?(.failure(error))

        // Then
        XCTAssertTrue(viewModel.isPresentingError)
        XCTAssertEqual(viewModel.presentableError, expectedErrorMessage)
    }

    func testDidTapAddCustom_ShowsDerivationPath() {
        // When
        viewModel.didTapAddCustom()

        // Then
        XCTAssertTrue(viewModel.isPresentingDerivationPath)
    }

    func testOnInfoBoxTap_PresentsTutorial() {
        // When
        viewModel.onInfoBoxTap()

        // Then
        XCTAssertTrue(viewModel.isNetworkTutorialPresented)
    }

    func testIsNetworkTutorialPresented_updatedToFalse_updatesNetworks() {
        // Given
        viewModel.isNetworkTutorialPresented = true

        // When
        viewModel.isNetworkTutorialPresented = false

        // Then
        XCTAssertEqual(networkServiceMock.getNetworksCallsCount, 2)
    }
}
