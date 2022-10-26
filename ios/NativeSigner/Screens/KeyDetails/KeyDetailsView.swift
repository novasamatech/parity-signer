//
//  KeyDetailsView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 29/08/2022.
//

import SwiftUI

struct KeyDetailsView: View {
    @State private var shouldPresentRemoveConfirmationModal = false
    @State private var shouldPresentBackupModal = false
    @State private var shouldPresentSelectionOverlay = false
    @State private var isShowingActionSheet = false
    @State private var isShowingRemoveConfirmation: Bool = false
    @State private var isShowingBackupModal: Bool = false
    @State private var isPresentingConnectivityAlert = false
    @State private var isPresentingSelectionOverlay = false
    @State private var isShowingKeysExportModal = false

    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var connectivityMediator: ConnectivityMediator
    @EnvironmentObject private var data: SignerDataModel

    @State var selectedSeeds: [String] = []

    // This view is recreated few times because of Rust navigation, for now we need to store modal view model in static
    // property because it can't be created earlier as it would trigger passcode request on the device
    private static var backupModalViewModel: BackupModalViewModel!
    private let alertClosure: (() -> Void)?
    private let viewModel: KeyDetailsViewModel
    private let actionModel: KeyDetailsActionModel
    private let forgetKeyActionHandler: ForgetKeySetAction
    private let exportPrivateKeyService: PrivateKeyQRCodeService
    private let resetWarningAction: ResetConnectivtyWarningsAction

    init(
        forgetKeyActionHandler: ForgetKeySetAction,
        viewModel: KeyDetailsViewModel,
        actionModel: KeyDetailsActionModel,
        exportPrivateKeyService: PrivateKeyQRCodeService,
        resetWarningAction: ResetConnectivtyWarningsAction,
        alertClosure: (() -> Void)? = nil
    ) {
        self.forgetKeyActionHandler = forgetKeyActionHandler
        self.viewModel = viewModel
        self.actionModel = actionModel
        self.exportPrivateKeyService = exportPrivateKeyService
        self.resetWarningAction = resetWarningAction
        self.alertClosure = alertClosure
    }

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
                        isShowingActionSheet.toggle()
                    })
                )
                // List
                mainList
            }
            .background(Asset.backgroundPrimary.swiftUIColor)
            if isPresentingSelectionOverlay {
                selectKeysOverlay
            } else {
                PrimaryButton(
                    action: {
                        navigation.perform(navigation: actionModel.createDerivedKey)
                    },
                    text: Localizable.KeyDetails.Action.create.key
                )
                .padding(Spacing.large)
            }
        }
        .fullScreenCover(
            isPresented: $isShowingActionSheet,
            onDismiss: {
                if shouldPresentRemoveConfirmationModal {
                    shouldPresentRemoveConfirmationModal.toggle()
                    isShowingRemoveConfirmation.toggle()
                }
                if shouldPresentBackupModal {
                    shouldPresentBackupModal.toggle()
                    if data.alert {
                        DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) {
                            isPresentingConnectivityAlert.toggle()
                        }
                    } else {
                        KeyDetailsView.backupModalViewModel = exportPrivateKeyService.backupViewModel()
                        isShowingBackupModal.toggle()
                    }
                }
                if shouldPresentSelectionOverlay {
                    shouldPresentSelectionOverlay.toggle()
                    isPresentingSelectionOverlay.toggle()
                }
            }
        ) {
            KeyDetailsActionsModal(
                isShowingActionSheet: $isShowingActionSheet,
                shouldPresentRemoveConfirmationModal: $shouldPresentRemoveConfirmationModal,
                shouldPresentBackupModal: $shouldPresentBackupModal,
                shouldPresentSelectionOverlay: $shouldPresentSelectionOverlay
            )
            .clearModalBackground()
        }
        .fullScreenCover(isPresented: $isShowingRemoveConfirmation) {
            HorizontalActionsBottomModal(
                viewModel: .forgetKeySet,
                mainAction: forgetKeyActionHandler.forgetKeySet(actionModel.removeSeed),
                // We need to fake right button action here or Rust machine will break
                // In old UI, if you dismiss equivalent of this modal, underlying modal would still be there,
                // so we need to inform Rust we actually hid it
                dismissAction: { _ = navigation.performFake(navigation: .init(action: .rightButtonAction)) }(),
                isShowingBottomAlert: $isShowingRemoveConfirmation
            )
            .clearModalBackground()
        }
        .fullScreenCover(isPresented: $isShowingBackupModal) {
            BackupModal(
                isShowingBackupModal: $isShowingBackupModal,
                viewModel: KeyDetailsView.backupModalViewModel
            )
            .clearModalBackground()
        }
        .fullScreenCover(
            isPresented: $isPresentingConnectivityAlert,
            onDismiss: checkForActionsPresentation
        ) {
            ErrorBottomModal(
                viewModel: connectivityMediator.isConnectivityOn ? .connectivityOn() : .connectivityWasOn(
                    continueAction: {
                        resetWarningAction.resetConnectivityWarnings()
                        shouldPresentBackupModal.toggle()
                    }()
                ),
                isShowingBottomAlert: $isPresentingConnectivityAlert
            )
            .clearModalBackground()
        }
        .fullScreenCover(
            isPresented: $isShowingKeysExportModal
        ) {
            ExportMultipleKeysModal(
                viewModel: .init(
                    viewModel: ExportMultipleKeysModalViewModel(
                        selectedItems: .keys(
                            key: selectedSeeds.contains(viewModel.keySummary.keyName) ? viewModel.keySummary : nil,
                            derivedKeys: viewModel.derivedKeys.filter { selectedSeeds.contains($0.viewModel.seedName) }
                        ),
                        seedNames: selectedSeeds
                    ),
                    isPresented: $isShowingKeysExportModal
                )
            )
            .clearModalBackground()
            .onAppear {
                selectedSeeds.removeAll()
                isPresentingSelectionOverlay.toggle()
            }
        }
    }

    func checkForActionsPresentation() {
        if shouldPresentRemoveConfirmationModal {
            shouldPresentRemoveConfirmationModal.toggle()
            isShowingRemoveConfirmation.toggle()
        }
        if shouldPresentBackupModal {
            shouldPresentBackupModal.toggle()
            if data.alert {
                DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) {
                    isPresentingConnectivityAlert.toggle()
                }
            } else {
                KeyDetailsView.backupModalViewModel = exportPrivateKeyService.backupViewModel()
                isShowingBackupModal.toggle()
            }
        }
    }

    var mainList: some View {
        List {
            // Main key cell
            KeySummaryView(
                viewModel: viewModel.keySummary,
                selectedSeeds: $selectedSeeds,
                isPresentingSelectionOverlay: $isPresentingSelectionOverlay
            )
            .padding(Padding.detailsCell)
            .keyDetailsListElement()
            .onTapGesture {
                if isPresentingSelectionOverlay {
                    let seedName = viewModel.keySummary.keyName
                    if selectedSeeds.contains(seedName) {
                        selectedSeeds.removeAll { $0 == seedName }
                    } else {
                        selectedSeeds.append(seedName)
                    }
                } else {
                    navigation.perform(navigation: actionModel.addressKeyNavigation)
                }
            }
            // Header
            HStack {
                Localizable.KeyDetails.Label.derived.text
                    .font(Fontstyle.bodyM.base)
                Spacer().frame(maxWidth: .infinity)
                Asset.switches.swiftUIImage
                    .frame(width: Heights.actionSheetButton)
                    .onTapGesture {
                        navigation.perform(navigation: .init(action: .networkSelector))
                    }
            }
            .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
            .padding(Padding.detailsCell)
            .keyDetailsListElement()
            // List of derived keys
            ForEach(
                viewModel.derivedKeys,
                id: \.viewModel.path
            ) { deriveKey in
                DerivedKeyRow(
                    viewModel: deriveKey.viewModel,
                    selectedSeeds: $selectedSeeds,
                    isPresentingSelectionOverlay: $isPresentingSelectionOverlay
                )
                .keyDetailsListElement()
                .onTapGesture {
                    if isPresentingSelectionOverlay {
                        let seedName = deriveKey.viewModel.seedName
                        if selectedSeeds.contains(seedName) {
                            selectedSeeds.removeAll { $0 == seedName }
                        } else {
                            selectedSeeds.append(seedName)
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

    var selectKeysOverlay: some View {
        VStack {
            // Top overlay
            HStack {
                NavbarButton(action: { isPresentingSelectionOverlay.toggle() }, icon: Image(.xmark))
                    .padding(.leading, Spacing.extraSmall)
                Spacer()
                Text(selectionTitle)
                    .font(Fontstyle.titleS.base)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor).lineLimit(1)
                Spacer()
                Spacer().frame(width: Heights.navigationButton)
            }
            .frame(height: Heights.tabbarHeight)
            .background(Asset.backgroundSecondary.swiftUIColor)
            Spacer()
            // Bottom overlay
            HStack {
                // Select All
                Button(action: { selectAll() }) {
                    Localizable.KeyDetails.Overlay.Action.selectAll.text
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .font(Fontstyle.labelL.base)
                }
                .padding(.leading, Spacing.medium)
                Spacer()
                // Export
                Button(action: { isShowingKeysExportModal.toggle() }) {
                    Localizable.KeyDetails.Overlay.Action.export.text
                        .foregroundColor(
                            selectedSeeds.isEmpty ? Asset.textAndIconsDisabled.swiftUIColor : Asset
                                .textAndIconsPrimary.swiftUIColor
                        )
                        .font(Fontstyle.labelL.base)
                }
                .padding(.trailing, Spacing.medium)
                .disabled(selectedSeeds.isEmpty)
            }
            .frame(height: Heights.tabbarHeight)
            .background(Asset.backgroundSecondary.swiftUIColor)
        }
    }

    var selectionTitle: String {
        let localizable = Localizable.KeyDetails.Overlay.Label.self
        let itemsCount = selectedSeeds.count
        let keyString = itemsCount == 1 ? localizable.Key.single.string : localizable.Key.plural.string
        return localizable.title(String(itemsCount), keyString)
    }

    func selectAll() {
        selectedSeeds = viewModel.derivedKeys.map(\.viewModel.seedName) +
            [viewModel.keySummary.keyName]
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
    @Binding var selectedSeeds: [String]
    @Binding var isPresentingSelectionOverlay: Bool

    private var isItemSelected: Bool {
        selectedSeeds.contains(viewModel.keyName)
    }

    var body: some View {
        HStack {
            VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                Text(viewModel.keyName)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    .font(Fontstyle.titleL.base)
                Text(viewModel.base58.truncateMiddle())
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(Fontstyle.bodyM.base)
                    .lineLimit(1)
            }
            Spacer()
            if isPresentingSelectionOverlay {
                isItemSelected ? Asset.checkmarkChecked.swiftUIImage : Asset.checkmarkUnchecked.swiftUIImage
            } else {
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
                    forgetKeyActionHandler: .init(navigation: NavigationCoordinator()),
                    viewModel: .init(
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
                                    base58: "15Gsc678654FDSG0HA04H0A",
                                    seedName: "name"
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
                                    base58: "15Gsc678654FDSG0HA04H0A",
                                    seedName: "name"
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
                                    base58: "15Gsc678654FDSG0HA04H0A",
                                    seedName: "name"
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
                                    base58: "15Gsc678654FDSG0HA04H0A",
                                    seedName: "name"
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
                                    base58: "15Gsc678654FDSG0HA04H0A",
                                    seedName: "name"
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
                                    base58: "15Gsc678654FDSG0HA04H0A",
                                    seedName: "name"
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
                                    base58: "15Gsc6786546423FDSG0HA04H0A",
                                    seedName: "name"
                                ),
                                actionModel: DerivedKeyActionModel(
                                    tapAction: .init(action: .rightButtonAction)
                                )
                            )
                        ]
                    ),
                    actionModel: KeyDetailsActionModel(
                        addressKeyNavigation: .init(action: .goBack),
                        derivedKeysNavigation: [],
                        alertClosure: nil,
                        removeSeed: ""
                    ),
                    exportPrivateKeyService: PrivateKeyQRCodeService(
                        navigation: NavigationCoordinator(),
                        keys: PreviewData.mkeys
                    ),
                    resetWarningAction: ResetConnectivtyWarningsAction(alert: Binding<Bool>.constant(false))
                )
            }
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
            .environmentObject(NavigationCoordinator())
        }
    }
#endif
