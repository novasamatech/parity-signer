//
//  KeyDetailsView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 29/08/2022.
//

import SwiftUI

struct KeyDetailsView: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var connectivityMediator: ConnectivityMediator
    @EnvironmentObject private var data: SharedDataModel
    @EnvironmentObject private var appState: AppState

    let forgetKeyActionHandler: ForgetKeySetAction
    let resetWarningAction: ResetConnectivtyWarningsAction

    var body: some View {
        ZStack(alignment: .bottom) {
            VStack(spacing: 0) {
                // Navigation bar
                NavigationBarView(
                    viewModel: .init(
                        leftButtons: [.init(type: .arrow, action: viewModel.onBackTap)],
                        rightButtons: [
                            .init(type: .plus, action: viewModel.onCreateDerivedKeyTap),
                            .init(type: .more, action: { viewModel.isShowingActionSheet.toggle() })
                        ]
                    )
                )
                switch viewModel.viewState {
                case .list:
                    ScrollView(showsIndicators: false) {
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
                        .padding(.top, Spacing.medium)
                        // List
                        mainList
                    }
                case .emptyState:
                    rootKeyHeader()
                    Spacer()
                    emptyState()
                    Spacer()
                }
            }
            .background(Asset.backgroundPrimary.swiftUIColor)
            if viewModel.isPresentingSelectionOverlay {
                selectKeysOverlay
            } else {
                VStack(spacing: 0) {
                    ConnectivityAlertOverlay(
                        viewModel: .init(resetWarningAction: ResetConnectivtyWarningsAction(
                            alert: $data
                                .alert
                        ))
                    )
                }
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
                    viewModel.selectedKeys.removeAll()
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
        .fullScreenCover(
            isPresented: $viewModel.isPresentingError
        ) {
            ErrorBottomModal(
                viewModel: viewModel.presentableError,
                isShowingBottomAlert: $viewModel.isPresentingError
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
                    selectedKeys: $viewModel.selectedKeys,
                    isPresentingSelectionOverlay: $viewModel.isPresentingSelectionOverlay
                )
                .contentShape(Rectangle())
                .onTapGesture {
                    viewModel.onDerivedKeyTap(deriveKey)
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
            .contentShape(Rectangle())
            .onTapGesture { viewModel.onRootKeyTap() }
        } else {
            EmptyView()
        }
    }

    @ViewBuilder
    func emptyState() -> some View {
        VStack(spacing: 0) {
            Localizable.KeyDetails.Label.EmptyState.header.text
                .font(PrimaryFont.titleM.font)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .padding(.top, Spacing.large)
                .padding(.horizontal, Spacing.componentSpacer)
            PrimaryButton(
                action: viewModel.onCreateDerivedKeyTap,
                text: Localizable.KeyDetails.Label.EmptyState.action.key,
                style: .secondary()
            )
            .padding(Spacing.large)
        }
        .containerBackground(CornerRadius.large, state: .actionableInfo)
        .padding(.horizontal, Spacing.medium)
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
                .padding(.top, Spacing.medium)
                .padding(.bottom, Spacing.extraSmall)
            HStack {
                Text(viewModel.base58.truncateMiddle())
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(PrimaryFont.bodyL.font)
                    .lineLimit(1)
                Asset.chevronDown.swiftUIImage
                    .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
            }
        }
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
//            .environmentObject(SharedDataModel())
//            .environmentObject(AppState())
//        }
//    }
// #endif
