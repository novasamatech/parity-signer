//
//  CreateKeysForNetworksViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 25/01/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import SwiftUI
import XCTest

final class CreateKeysForNetworksViewModelTests: XCTestCase {
    private var viewModel: CreateKeysForNetworksView.ViewModel!
    private var networkServiceMock: GetManagedNetworksServicingMock!
    private var createKeySetServiceMock: CreateKeySetServicingMock!
    private var createKeyServiceMock: CreateDerivedKeyServicingMock!
    private var recoveryKeySetServiceMock: RecoverKeySetServicingMock!
    private var seedsMediatorMock: SeedsMediatingMock!
    private var isPresented: Bool = false
    private var onCompletionActionExecuted: CreateKeysForNetworksView.OnCompletionAction?
    private var seedPhrase: String!
    private var seedName: String!

    override func setUp() {
        super.setUp()
        seedPhrase = "SeedPhrase"
        seedName = "SeedName"
        networkServiceMock = GetManagedNetworksServicingMock()
        createKeySetServiceMock = CreateKeySetServicingMock()
        createKeyServiceMock = CreateDerivedKeyServicingMock()
        recoveryKeySetServiceMock = RecoverKeySetServicingMock()
        seedsMediatorMock = SeedsMediatingMock()
        seedsMediatorMock.checkSeedPhraseCollisionSeedPhraseReturnValue = false
        seedsMediatorMock.createSeedSeedNameSeedPhraseShouldCheckForCollisionReturnValue = true
        viewModel = createViewModel(mode: .bananaSplit)
    }

    override func tearDown() {
        viewModel = nil
        networkServiceMock = nil
        createKeySetServiceMock = nil
        createKeyServiceMock = nil
        recoveryKeySetServiceMock = nil
        seedsMediatorMock = nil
        isPresented = false
        onCompletionActionExecuted = nil
        super.tearDown()
    }

    func test_init_updatesNetworks() {
        // Given

        // Then
        XCTAssertEqual(networkServiceMock.getNetworksCallsCount, 1)
    }

    func test_init_updatesNetworks_whenPreselectedNetworksAvailable_selectsThem() {
        // Given
        let availableNetworks: [MmNetwork] = [
            .generate(title: "polkadot"),
            .generate(title: "kusama"),
            .generate(title: "moonbeam")
        ]
        networkServiceMock.getNetworksReceivedCompletion.first?(.success(availableNetworks))

        // Then
        XCTAssertEqual(viewModel.selectedNetworks.count, 2)
    }

    func test_step_whenBananaSplit_returnsTwo() {
        // Given
        viewModel = createViewModel(mode: .bananaSplit)

        // Then
        XCTAssertEqual(viewModel.step, 2)
    }

    func test_step_whenCreateKeySet_returnsThree() {
        // Given
        viewModel = createViewModel(mode: .createKeySet)

        // Then
        XCTAssertEqual(viewModel.step, 3)
    }

    func test_step_whenRecoverKeySet_returnsThree() {
        // Given
        viewModel = createViewModel(mode: .recoverKeySet)

        // Then
        XCTAssertEqual(viewModel.step, 3)
    }

    func test_SelectAllNetworks_SelectsAllAvailableNetworks() {
        // Given
        let networks = [MmNetwork.generate(), MmNetwork.generate()]
        networkServiceMock.getNetworksReceivedCompletion.first?(.success(networks))

        // When
        viewModel.selectAllNetworks()

        // Then
        XCTAssertEqual(viewModel.selectedNetworks, networks)
    }

    func test_IsSelected_ReturnsTrueForSelectedNetwork() {
        // Given
        let network = MmNetwork.generate()
        viewModel.selectedNetworks.append(network)

        // When / Then
        XCTAssertTrue(viewModel.isSelected(network))
    }

    func test_ToggleSelection_RemovesNetworkIfSelected() {
        // Given
        let network = MmNetwork.generate()
        viewModel.selectedNetworks.append(network)

        // When
        viewModel.toggleSelection(network)

        // Then
        XCTAssertTrue(viewModel.selectedNetworks.isEmpty)
    }

    func test_ToggleSelection_AddsNetworkIfNotSelected() {
        // Given
        let network = MmNetwork.generate()

        // When
        viewModel.toggleSelection(network)

        // Then
        XCTAssertEqual(viewModel.selectedNetworks, [network])
    }

    func testOnKeyCreationComplete_SetsIsPresentedToFalse() {
        // Given
        viewModel = createViewModel(mode: .createKeySet)

        // When
        viewModel.onKeyCreationComplete()

        // Then
        XCTAssertFalse(viewModel.isPresented)
    }

    func testOnCreateEmptyKeySetTap_InvokesContinueKeySetAction() {
        // Given
        viewModel = createViewModel(mode: .createKeySet)

        // When
        viewModel.onCreateEmptyKeySetTap()

        // Then
        XCTAssertEqual(createKeySetServiceMock.confirmKeySetCreationSeedNameSeedPhraseNetworksCallsCount, 1)
    }

    func test_OnDoneTap_WhenNoNetworksSelected_PresentsConfirmation() {
        // When
        viewModel.onDoneTap()

        // Then
        XCTAssertTrue(viewModel.isPresentingConfirmation)
    }

    func testOnDoneTap_WhenNetworksSelected_checksSeedPhraseCollision() {
        // Given
        let network = MmNetwork.generate()
        viewModel.selectedNetworks.append(network)

        // When
        viewModel.onDoneTap()

        // Then
        XCTAssertEqual(seedsMediatorMock.checkSeedPhraseCollisionSeedPhraseCallsCount, 1)
        XCTAssertEqual(seedsMediatorMock.checkSeedPhraseCollisionSeedPhraseReceivedSeedPhrase, [seedPhrase])
    }

    func testOnDoneTap_WhenCollisionOccurs_ShowsError() {
        // Given
        viewModel = createViewModel(mode: .createKeySet)
        viewModel.selectedNetworks = [.generate()]
        seedsMediatorMock.checkSeedPhraseCollisionSeedPhraseReturnValue = true

        // When
        viewModel.onDoneTap()

        // Then
        XCTAssertTrue(viewModel.isPresentingError)
        XCTAssertEqual(viewModel.errorViewModel, .seedPhraseAlreadyExists())
    }

    func testOnDoneTap_WhenNoCollision_createsSeedWithExpectedParameters() {
        // Given
        viewModel = createViewModel(mode: .createKeySet)
        viewModel.selectedNetworks = [MmNetwork.generate()]

        // When
        viewModel.onDoneTap()

        // Then
        XCTAssertEqual(seedsMediatorMock.createSeedSeedNameSeedPhraseShouldCheckForCollisionCallsCount, 1)
        XCTAssertEqual(
            seedsMediatorMock.createSeedSeedNameSeedPhraseShouldCheckForCollisionReceivedSeedName,
            [seedName]
        )
        XCTAssertEqual(
            seedsMediatorMock.createSeedSeedNameSeedPhraseShouldCheckForCollisionReceivedSeedPhrase,
            [seedPhrase]
        )
        XCTAssertEqual(
            seedsMediatorMock.createSeedSeedNameSeedPhraseShouldCheckForCollisionReceivedShouldCheckForCollision,
            [false]
        )
    }

    func testOnDoneTap_WhenNoCollision_confirmsKeySetCreation() {
        // Given
        viewModel = createViewModel(mode: .createKeySet)
        viewModel.selectedNetworks = [.generate()]
        seedsMediatorMock.checkSeedPhraseCollisionSeedPhraseReturnValue = false

        // When
        viewModel.onDoneTap()

        // Then
        XCTAssertEqual(createKeySetServiceMock.confirmKeySetCreationSeedNameSeedPhraseNetworksCallsCount, 1)
        XCTAssertEqual(
            createKeySetServiceMock.confirmKeySetCreationSeedNameSeedPhraseNetworksReceivedNetworks,
            [viewModel.selectedNetworks]
        )
        XCTAssertEqual(
            createKeySetServiceMock.confirmKeySetCreationSeedNameSeedPhraseNetworksReceivedSeedName,
            [seedName]
        )
        XCTAssertEqual(
            createKeySetServiceMock.confirmKeySetCreationSeedNameSeedPhraseNetworksReceivedSeedPhrase,
            [seedPhrase]
        )
    }

    func testOnDoneTap_WhenNoCollision_whenSuccess_callsCompletionAndHidesView() {
        // Given
        viewModel = createViewModel(mode: .createKeySet)
        viewModel.selectedNetworks = [.generate()]
        seedsMediatorMock.checkSeedPhraseCollisionSeedPhraseReturnValue = false

        // When
        viewModel.onDoneTap()
        createKeySetServiceMock.confirmKeySetCreationSeedNameSeedPhraseNetworksReceivedCompletion.first?(.success(()))

        // Then
        XCTAssertFalse(viewModel.isPresented)
        XCTAssertEqual(onCompletionActionExecuted, .createKeySet(seedName: seedName))
    }

    func testOnDoneTap_WhenNoCollision_whenFailure_callsCompletionAndHidesView() {
        // Given
        let error = ServiceError(message: "message")
        viewModel = createViewModel(mode: .createKeySet)
        viewModel.selectedNetworks = [.generate()]
        seedsMediatorMock.checkSeedPhraseCollisionSeedPhraseReturnValue = false

        // When
        viewModel.onDoneTap()
        createKeySetServiceMock.confirmKeySetCreationSeedNameSeedPhraseNetworksReceivedCompletion
            .first?(.failure(error))

        // Then
        XCTAssertEqual(viewModel.errorViewModel, .alertError(message: error.localizedDescription))
        XCTAssertTrue(viewModel.isPresentingError)
    }

    func testTitle_ForCreateKeySetMode_ReturnsCreateTitle() {
        // Given
        viewModel = createViewModel(mode: .createKeySet)

        // When
        let title = viewModel.title()

        // Then
        XCTAssertEqual(title, Localizable.CreateKeysForNetwork.Label.Title.create.string)
    }

    func testTitle_ForRecoverKeySetMode_ReturnsRecoverTitle() {
        // Given
        viewModel = createViewModel(mode: .recoverKeySet)

        // When
        let title = viewModel.title()

        // Then
        XCTAssertEqual(title, Localizable.CreateKeysForNetwork.Label.Title.recover.string)
    }

    func testHeader_ForCreateKeySetMode_ReturnsCreateHeader() {
        // Given
        viewModel = createViewModel(mode: .createKeySet)

        // When
        let header = viewModel.header()

        // Then
        XCTAssertEqual(header, Localizable.CreateKeysForNetwork.Label.Header.create.string)
    }

    func testHeader_ForRecoverKeySetMode_ReturnsRecoverHeader() {
        // Given
        viewModel = createViewModel(mode: .recoverKeySet)

        // When
        let header = viewModel.header()

        // Then
        XCTAssertEqual(header, Localizable.CreateKeysForNetwork.Label.Header.recover.string)
    }

    private func createViewModel(mode: CreateKeysForNetworksView.Mode) -> CreateKeysForNetworksView.ViewModel {
        CreateKeysForNetworksView.ViewModel(
            seedName: seedName,
            seedPhrase: seedPhrase,
            mode: mode,
            networkService: networkServiceMock,
            createKeyService: createKeyServiceMock,
            createKeySetService: createKeySetServiceMock,
            recoveryKeySetService: recoveryKeySetServiceMock,
            seedsMediator: seedsMediatorMock,
            isPresented: Binding(get: { self.isPresented }, set: { self.isPresented = $0 }),
            onCompletion: { [weak self] action in
                self?.onCompletionActionExecuted = action
            }
        )
    }
}
