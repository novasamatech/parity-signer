//
//  KeyDetailsView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 29/08/2022.
//

import SwiftUI

struct KeyDetailsView: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var connectivityMediator: ConnectivityMediator
    @EnvironmentObject private var data: SignerDataModel
    @EnvironmentObject private var appState: AppState

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
                ScrollView {
                    // Main key cell
                    rootKeyHeader()
                    // Derived Keys header
                    HStack {
                        Localizable.KeyDetails.Label.derived.text
                            .font(PrimaryFont.bodyM.font)
                        Spacer().frame(maxWidth: .infinity)
                        Asset.switches.swiftUIImage
                            .foregroundColor(
                                viewModel.isFilteringActive ? Asset.accentPink300.swiftUIColor : Asset
                                    .textAndIconsTertiary.swiftUIColor
                            )
                            .frame(width: Heights.actionSheetButton)
                            .onTapGesture {
                                viewModel.onNetworkSelectionTap()
                            }
                    }
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .padding(.horizontal, Spacing.large)
                    // List
                    mainList
                }
            }
            .background(Asset.backgroundPrimary.swiftUIColor)
            if viewModel.isPresentingSelectionOverlay {
                selectKeysOverlay
            } else {
                PrimaryButton(
                    action: {
                        navigation.perform(navigation: viewModel.createDerivedKey)
                    },
                    text: Localizable.KeyDetails.Action.create.key
                )
                .padding(Spacing.large)
            }
        }
        .onAppear {
            viewModel.use(navigation: navigation)
            viewModel.use(appState: appState)
            viewModel.refreshData()
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
                mainAction: forgetKeyActionHandler.forgetKeySet(viewModel.removeSeed),
                // We need to fake right button action here or Rust machine will break
                // In old UI, if you dismiss equivalent of this modal, underlying modal would still be there,
                // so we need to inform Rust we actually hid it
                dismissAction: { _ = navigation.performFake(navigation: .init(action: .rightButtonAction)) }(),
                isShowingBottomAlert: $viewModel.isShowingRemoveConfirmation
            )
            .clearModalBackground()
        }
        .fullScreenCover(isPresented: $viewModel.isShowingBackupModal) {
            if let viewModel = viewModel.backupViewModel() {
                BackupModal(
                    isShowingBackupModal: $viewModel.isShowingBackupModal,
                    viewModel: viewModel
                )
                .clearModalBackground()
            } else {
                EmptyView()
            }
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
            if let keyExportModel = viewModel.keyExportModel() {
                ExportMultipleKeysModal(
                    viewModel: .init(
                        viewModel: keyExportModel,
                        isPresented: $viewModel.isShowingKeysExportModal
                    )
                )
                .clearModalBackground()
                .onAppear {
                    viewModel.selectedSeeds.removeAll()
                    viewModel.isPresentingSelectionOverlay.toggle()
                }
            } else {
                EmptyView()
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
        .fullScreenCover(
            isPresented: $viewModel.isPresentingRootDetails
        ) {
            RootKeyDetailsModal(
                isPresented: $viewModel.isPresentingRootDetails,
                viewModel: viewModel.rootKeyDetails()
            )
            .clearModalBackground()
        }
    }

    var mainList: some View {
        LazyVStack(spacing: 0) {
            // List of derived keys
            ForEach(
                viewModel.derivedKeys,
                id: \.viewModel.addressKey
            ) { deriveKey in
                DerivedKeyRow(
                    viewModel: deriveKey.viewModel,
                    selectedSeeds: $viewModel.selectedSeeds,
                    isPresentingSelectionOverlay: $viewModel.isPresentingSelectionOverlay
                )
                .contentShape(Rectangle())
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
                .frame(height: Heights.actionButton + Spacing.large)
        }
    }

    @ViewBuilder
    func rootKeyHeader() -> some View {
        if let keySummary = viewModel.keySummary {
            KeySummaryView(
                viewModel: keySummary,
                isPresentingSelectionOverlay: $viewModel.isPresentingSelectionOverlay
            )
            .padding(Padding.detailsCell)
            .contentShape(Rectangle())
            .onTapGesture { viewModel.onRootKeyTap() }
        } else {
            EmptyView()
        }
    }
}

private struct KeySummaryView: View {
    let viewModel: KeySummaryViewModel
    @Binding var isPresentingSelectionOverlay: Bool

    var body: some View {
        VStack(alignment: .center, spacing: Spacing.extraExtraSmall) {
            Text(viewModel.keyName)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(PrimaryFont.titleXL.font)
            HStack {
                Text(viewModel.base58.truncateMiddle())
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(PrimaryFont.bodyL.font)
                    .lineLimit(1)
                Asset.chevronDown.swiftUIImage
                    .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
            }
        }
        .padding(.vertical, Spacing.medium)
        .padding(.horizontal, Spacing.large)
    }
}

// #if DEBUG
//    struct KeyDetailsView_Previews: PreviewProvider {
//        static var previews: some View {
//            VStack {
//                KeyDetailsView(
//                    dataModel: .init(
//                        keySummary: KeySummaryViewModel(
//                            keyName: "Main Polkadot",
//                            base58: "15Gsc678...0HA04H0A"
//                        ),
//                        derivedKeys: [
//                            DerivedKeyRowModel(
//                                viewModel: DerivedKeyRowViewModel(
//                                    identicon: PreviewData.exampleIdenticon,
//                                    path: "// polkadot",
//                                    hasPassword: false,
//                                    base58: "15Gsc678654FDSG0HA04H0A"
//                                ),
//                                actionModel: DerivedKeyActionModel(
//                                    tapAction: .init(action: .rightButtonAction)
//                                )
//                            ),
//                            DerivedKeyRowModel(
//                                viewModel: DerivedKeyRowViewModel(
//                                    identicon: PreviewData.exampleIdenticon,
//                                    path: "// polkadot",
//                                    hasPassword: false,
//                                    base58: "15Gsc678654FDSG0HA04H0A"
//                                ),
//                                actionModel: DerivedKeyActionModel(
//                                    tapAction: .init(action: .rightButtonAction)
//                                )
//                            ),
//                            DerivedKeyRowModel(
//                                viewModel: DerivedKeyRowViewModel(
//                                    identicon: PreviewData.exampleIdenticon,
//                                    path: "//astar//verylongpathsolongitrequirestwolinesoftextormaybeevenmore",
//                                    hasPassword: true,
//                                    base58: "15Gsc678654FDSG0HA04H0A"
//                                ),
//                                actionModel: DerivedKeyActionModel(
//                                    tapAction: .init(action: .rightButtonAction)
//                                )
//                            ),
//                            DerivedKeyRowModel(
//                                viewModel: DerivedKeyRowViewModel(
//                                    identicon: PreviewData.exampleIdenticon,
//                                    path: "//verylongpathsolongitrequirestwolinesoftextormaybeevenmore",
//                                    hasPassword: false,
//                                    base58: "15Gsc678654FDSG0HA04H0A"
//                                ),
//                                actionModel: DerivedKeyActionModel(
//                                    tapAction: .init(action: .rightButtonAction)
//                                )
//                            ),
//                            DerivedKeyRowModel(
//                                viewModel: DerivedKeyRowViewModel(
//                                    identicon: PreviewData.exampleIdenticon,
//                                    path: "// acala",
//                                    hasPassword: true,
//                                    base58: "15Gsc678654FDSG0HA04H0A"
//                                ),
//                                actionModel: DerivedKeyActionModel(
//                                    tapAction: .init(action: .rightButtonAction)
//                                )
//                            ),
//                            DerivedKeyRowModel(
//                                viewModel: DerivedKeyRowViewModel(
//                                    identicon: PreviewData.exampleIdenticon,
//                                    path: "// moonbeam",
//                                    hasPassword: true,
//                                    base58: "15Gsc678654FDSG0HA04H0A"
//                                ),
//                                actionModel: DerivedKeyActionModel(
//                                    tapAction: .init(action: .rightButtonAction)
//                                )
//                            ),
//                            DerivedKeyRowModel(
//                                viewModel: DerivedKeyRowViewModel(
//                                    identicon: PreviewData.exampleIdenticon,
//                                    path: "// kilt",
//                                    hasPassword: true,
//                                    base58: "15Gsc6786546423FDSG0HA04H0A"
//                                ),
//                                actionModel: DerivedKeyActionModel(
//                                    tapAction: .init(action: .rightButtonAction)
//                                )
//                            )
//                        ]
//                    ),
//                    viewModel: .init(
//                        keysData: PreviewData.mKeyNew,
//                        exportPrivateKeyService: PrivateKeyQRCodeService(
//                            navigation: NavigationCoordinator(),
//                            keys: PreviewData.mkeys
//                        )
//                    ),
//                    forgetKeyActionHandler: .init(navigation: NavigationCoordinator()),
//                    resetWarningAction: ResetConnectivtyWarningsAction(alert: Binding<Bool>.constant(true))
//                )
//            }
//            .preferredColorScheme(.dark)
//            .previewLayout(.sizeThatFits)
//            .environmentObject(NavigationCoordinator())
//            .environmentObject(ConnectivityMediator())
//            .environmentObject(SignerDataModel())
//            .environmentObject(AppState())
//        }
//    }
// #endif
