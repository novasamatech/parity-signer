//
//  KeyDetailsView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 29/08/2022.
//

import SwiftUI

struct KeyDetailsView: View {
    let dataModel: KeyDetailsDataModel
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var connectivityMediator: ConnectivityMediator
    @EnvironmentObject private var data: SignerDataModel
    @EnvironmentObject private var appState: AppState

    // This view is recreated few times because of Rust navigation, for now we need to store modal view model in static
    // property because it can't be created earlier as it would trigger passcode request on the device
    static var backupModalViewModel: BackupModalViewModel!
    let forgetKeyActionHandler: ForgetKeySetAction
    let resetWarningAction: ResetConnectivtyWarningsAction

    var body: some View {
        ZStack(alignment: .bottom) {
            VStack(spacing: 0) {
                // Navigation bar
                NavigationBarView(
                    viewModel: .init(
                        leftButton: .arrow,
                        rightButton: .more
                    ),
                    actionModel: .init(rightBarMenuAction: {
                        viewModel.isShowingActionSheet.toggle()
                    })
                )
                // List
                mainList
            }
            .background(Asset.backgroundPrimary.swiftUIColor)
            if viewModel.isPresentingSelectionOverlay {
                selectKeysOverlay
            } else {
                PrimaryButton(
                    action: {
                        navigation.perform(navigation: dataModel.createDerivedKey)
                    },
                    text: Localizable.KeyDetails.Action.create.key
                )
                .padding(Spacing.large)
            }
        }
        .onAppear {
            viewModel.use(navigation: navigation)
            viewModel.use(appState: appState)
            viewModel.refreshData(dataModel: dataModel)
        }
        .fullScreenCover(
            isPresented: $viewModel.isShowingActionSheet,
            onDismiss: { viewModel.onActionSheetDismissal(data.alert) }
        ) {
            KeyDetailsActionsModal(
                isShowingActionSheet: $viewModel.isShowingActionSheet,
                shouldPresentRemoveConfirmationModal: $viewModel.shouldPresentRemoveConfirmationModal,
                shouldPresentBackupModal: $viewModel.shouldPresentBackupModal,
                shouldPresentSelectionOverlay: $viewModel.shouldPresentSelectionOverlay
            )
            .clearModalBackground()
        }
        .fullScreenCover(isPresented: $viewModel.isShowingRemoveConfirmation) {
            HorizontalActionsBottomModal(
                viewModel: .forgetKeySet,
                mainAction: forgetKeyActionHandler.forgetKeySet(dataModel.removeSeed),
                // We need to fake right button action here or Rust machine will break
                // In old UI, if you dismiss equivalent of this modal, underlying modal would still be there,
                // so we need to inform Rust we actually hid it
                dismissAction: { _ = navigation.performFake(navigation: .init(action: .rightButtonAction)) }(),
                isShowingBottomAlert: $viewModel.isShowingRemoveConfirmation
            )
            .clearModalBackground()
        }
        .fullScreenCover(isPresented: $viewModel.isShowingBackupModal) {
            BackupModal(
                isShowingBackupModal: $viewModel.isShowingBackupModal,
                viewModel: KeyDetailsView.backupModalViewModel
            )
            .clearModalBackground()
        }
        .fullScreenCover(
            isPresented: $viewModel.isPresentingConnectivityAlert,
            onDismiss: { viewModel.onActionSheetDismissal(data.alert) }
        ) {
            ErrorBottomModal(
                viewModel: connectivityMediator.isConnectivityOn ? .connectivityOn() : .connectivityWasOn(
                    continueAction: {
                        resetWarningAction.resetConnectivityWarnings()
                        viewModel.shouldPresentBackupModal.toggle()
                    }()
                ),
                isShowingBottomAlert: $viewModel.isPresentingConnectivityAlert
            )
            .clearModalBackground()
        }
        .fullScreenCover(
            isPresented: $viewModel.isShowingKeysExportModal
        ) {
            ExportMultipleKeysModal(
                viewModel: .init(
                    viewModel: viewModel.keyExportModel(dataModel: dataModel),
                    isPresented: $viewModel.isShowingKeysExportModal
                )
            )
            .clearModalBackground()
            .onAppear {
                viewModel.selectedSeeds.removeAll()
                viewModel.isPresentingSelectionOverlay.toggle()
            }
        }
        .fullScreenCover(
            isPresented: $viewModel.isPresentingNetworkSelection
        ) {
            NetworkSelectionModal(
                viewModel: .init(isPresented: $viewModel.isPresentingNetworkSelection)
            )
            .clearModalBackground()
        }
    }

    var mainList: some View {
        List {
            // Main key cell
            KeySummaryView(
                viewModel: dataModel.keySummary,
                isPresentingSelectionOverlay: $viewModel.isPresentingSelectionOverlay
            )
            .padding(Padding.detailsCell)
            .keyDetailsListElement()
            .onTapGesture {
                if !viewModel.isPresentingSelectionOverlay {
                    navigation.perform(navigation: dataModel.addressKeyNavigation)
                }
            }
            // Header
            HStack {
                Localizable.KeyDetails.Label.derived.text
                    .font(PrimaryFont.bodyM.font)
                Spacer().frame(maxWidth: .infinity)
                Asset.switches.swiftUIImage
                    .frame(width: Heights.actionSheetButton)
                    .onTapGesture {
                        viewModel.onNetworkSelectionTap()
                    }
            }
            .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
            .padding(Padding.detailsCell)
            .keyDetailsListElement()
            // List of derived keys
            ForEach(
                dataModel.derivedKeys,
                id: \.viewModel.addressKey
            ) { deriveKey in
                DerivedKeyRow(
                    viewModel: deriveKey.viewModel,
                    selectedSeeds: $viewModel.selectedSeeds,
                    isPresentingSelectionOverlay: $viewModel.isPresentingSelectionOverlay
                )
                .keyDetailsListElement()
                .onTapGesture {
                    if viewModel.isPresentingSelectionOverlay {
                        let seedName = deriveKey.viewModel.path
                        if viewModel.selectedSeeds.contains(seedName) {
                            viewModel.selectedSeeds.removeAll { $0 == seedName }
                        } else {
                            viewModel.selectedSeeds.append(seedName)
                        }
                    } else {
                        navigation.perform(navigation: deriveKey.actionModel.tapAction)
                    }
                }
            }
            Spacer()
                .keyDetailsListElement()
                .frame(height: Heights.actionButton + Spacing.large)
        }
        .listStyle(.plain)
        .hiddenScrollContent()
    }
}

private struct KeyDetailsListElement: ViewModifier {
    func body(content: Content) -> some View {
        content
            .listRowBackground(Asset.backgroundPrimary.swiftUIColor)
            .listRowSeparator(.hidden)
            .listRowInsets(EdgeInsets())
            .contentShape(Rectangle())
    }
}

private extension View {
    func keyDetailsListElement() -> some View {
        modifier(KeyDetailsListElement())
    }
}

private struct KeySummaryView: View {
    let viewModel: KeySummaryViewModel
    @Binding var isPresentingSelectionOverlay: Bool

    var body: some View {
        HStack {
            VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                Text(viewModel.keyName)
                    .foregroundColor(
                        isPresentingSelectionOverlay ? Asset.textAndIconsDisabled.swiftUIColor : Asset
                            .textAndIconsPrimary.swiftUIColor
                    )
                    .font(PrimaryFont.titleL.font)
                Text(viewModel.base58.truncateMiddle())
                    .foregroundColor(
                        isPresentingSelectionOverlay ? Asset.textAndIconsDisabled.swiftUIColor : Asset
                            .textAndIconsTertiary.swiftUIColor
                    )
                    .font(PrimaryFont.bodyM.font)
                    .lineLimit(1)
            }
            Spacer()
            if !isPresentingSelectionOverlay {
                Asset.chevronRight.swiftUIImage
                    .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
            }
        }
    }
}

#if DEBUG
    struct KeyDetailsView_Previews: PreviewProvider {
        static var previews: some View {
            VStack {
                KeyDetailsView(
                    dataModel: .init(
                        keySummary: KeySummaryViewModel(
                            keyName: "Main Polkadot",
                            base58: "15Gsc678...0HA04H0A"
                        ),
                        derivedKeys: [
                            DerivedKeyRowModel(
                                viewModel: DerivedKeyRowViewModel(
                                    identicon: PreviewData.exampleIdenticon,
                                    path: "// polkadot",
                                    hasPassword: false,
                                    base58: "15Gsc678654FDSG0HA04H0A"
                                ),
                                actionModel: DerivedKeyActionModel(
                                    tapAction: .init(action: .rightButtonAction)
                                )
                            ),
                            DerivedKeyRowModel(
                                viewModel: DerivedKeyRowViewModel(
                                    identicon: PreviewData.exampleIdenticon,
                                    path: "// polkadot",
                                    hasPassword: false,
                                    base58: "15Gsc678654FDSG0HA04H0A"
                                ),
                                actionModel: DerivedKeyActionModel(
                                    tapAction: .init(action: .rightButtonAction)
                                )
                            ),
                            DerivedKeyRowModel(
                                viewModel: DerivedKeyRowViewModel(
                                    identicon: PreviewData.exampleIdenticon,
                                    path: "//astar//verylongpathsolongitrequirestwolinesoftextormaybeevenmore",
                                    hasPassword: true,
                                    base58: "15Gsc678654FDSG0HA04H0A"
                                ),
                                actionModel: DerivedKeyActionModel(
                                    tapAction: .init(action: .rightButtonAction)
                                )
                            ),
                            DerivedKeyRowModel(
                                viewModel: DerivedKeyRowViewModel(
                                    identicon: PreviewData.exampleIdenticon,
                                    path: "//verylongpathsolongitrequirestwolinesoftextormaybeevenmore",
                                    hasPassword: false,
                                    base58: "15Gsc678654FDSG0HA04H0A"
                                ),
                                actionModel: DerivedKeyActionModel(
                                    tapAction: .init(action: .rightButtonAction)
                                )
                            ),
                            DerivedKeyRowModel(
                                viewModel: DerivedKeyRowViewModel(
                                    identicon: PreviewData.exampleIdenticon,
                                    path: "// acala",
                                    hasPassword: true,
                                    base58: "15Gsc678654FDSG0HA04H0A"
                                ),
                                actionModel: DerivedKeyActionModel(
                                    tapAction: .init(action: .rightButtonAction)
                                )
                            ),
                            DerivedKeyRowModel(
                                viewModel: DerivedKeyRowViewModel(
                                    identicon: PreviewData.exampleIdenticon,
                                    path: "// moonbeam",
                                    hasPassword: true,
                                    base58: "15Gsc678654FDSG0HA04H0A"
                                ),
                                actionModel: DerivedKeyActionModel(
                                    tapAction: .init(action: .rightButtonAction)
                                )
                            ),
                            DerivedKeyRowModel(
                                viewModel: DerivedKeyRowViewModel(
                                    identicon: PreviewData.exampleIdenticon,
                                    path: "// kilt",
                                    hasPassword: true,
                                    base58: "15Gsc6786546423FDSG0HA04H0A"
                                ),
                                actionModel: DerivedKeyActionModel(
                                    tapAction: .init(action: .rightButtonAction)
                                )
                            )
                        ]
                    ),
                    viewModel: .init(
                        keysData: PreviewData.mKeyNew,
                        exportPrivateKeyService: PrivateKeyQRCodeService(
                            navigation: NavigationCoordinator(),
                            keys: PreviewData.mkeys
                        )
                    ),
                    forgetKeyActionHandler: .init(navigation: NavigationCoordinator()),
                    resetWarningAction: ResetConnectivtyWarningsAction(alert: Binding<Bool>.constant(true))
                )
            }
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
            .environmentObject(NavigationCoordinator())
            .environmentObject(ConnectivityMediator())
            .environmentObject(SignerDataModel())
            .environmentObject(AppState())
        }
    }
#endif
