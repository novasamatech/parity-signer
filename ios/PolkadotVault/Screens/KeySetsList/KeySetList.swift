//
//  KeySetList.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 22/08/2022.
//

import SwiftUI

struct KeySetList: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject var appState: AppState

    var body: some View {
        NavigationView {
            ZStack(alignment: .bottom) {
                // Main screen
                VStack(spacing: 0) {
                    // Navigation Bar
                    NavigationBarView(
                        viewModel: NavigationBarViewModel(
                            title: .title(Localizable.KeySets.title.string),
                            leftButtons: [.init(
                                type: viewModel.isExportKeysSelected ? .xmark : .empty,
                                action: viewModel.onCancelExportTap
                            )],
                            rightButtons: [.init(
                                type: viewModel.isExportKeysSelected ? .empty : .more,
                                action: viewModel.onShowMoreTap
                            )],
                            backgroundColor: Asset.backgroundSystem.swiftUIColor
                        )
                    )
                    // Empty state
                    if viewModel.listViewModel.list.isEmpty {
                        KeyListEmptyState()
                    } else {
                        // List of Key Sets
                        keyList()
                    }
                }
                .background(
                    Asset.backgroundSystem.swiftUIColor
                        .ignoresSafeArea()
                )
                .navigationViewStyle(StackNavigationViewStyle())
                .navigationBarHidden(true)
                VStack {
                    // Add Key Set
                    if !viewModel.isExportKeysSelected {
                        VStack(spacing: 0) {
                            ConnectivityAlertOverlay(viewModel: .init())
                            PrimaryButton(
                                action: viewModel.onAddKeySetTap,
                                text: Localizable.KeySets.Action.add.key
                            )
                            .padding(.horizontal, Spacing.large)
                            .padding(.bottom, Spacing.large)
                        }
                    }
                    TabBarView(
                        viewModel: viewModel.tabBarViewModel
                    )
                }
                if viewModel.isExportKeysSelected {
                    exportKeysOverlay
                }
                // Navigation Links
                NavigationLink(
                    destination:
                    KeyDetailsView(viewModel: viewModel.keyDetails())
                        .navigationBarHidden(true),
                    isActive: $viewModel.isShowingDetails
                ) { EmptyView() }
            }
        }
        .onAppear {
            viewModel.use(appState: appState)
            viewModel.updateData()
        }
        .onChange(of: viewModel.isShowingDetails, perform: { _ in
            guard !viewModel.isShowingDetails else { return }
            viewModel.updateData()
        })
        .fullScreenModal(
            isPresented: $viewModel.isShowingNewSeedMenu,
            onDismiss: viewModel.onNewSeedMenuDismiss
        ) {
            AddKeySetModal(
                isShowingNewSeedMenu: $viewModel.isShowingNewSeedMenu,
                shouldShowCreateKeySet: $viewModel.shouldShowCreateKeySet,
                shouldShowRecoverKeySet: $viewModel.shouldShowRecoverKeySet
            )
            .clearModalBackground()
        }
        .fullScreenModal(
            isPresented: $viewModel.isShowingCreateKeySet,
            onDismiss: viewModel.updateData
        ) {
            EnterKeySetNameView(
                viewModel: .init(
                    isPresented: $viewModel.isShowingCreateKeySet,
                    onCompletion: viewModel.onKeySetAddCompletion(_:)
                )
            )
        }
        .fullScreenModal(
            isPresented: $viewModel.isShowingRecoverKeySet,
            onDismiss: viewModel.updateData
        ) {
            RecoverKeySetNameView(
                viewModel: .init(
                    isPresented: $viewModel.isShowingRecoverKeySet,
                    onCompletion: viewModel.onKeySetAddCompletion(_:)
                )
            )
        }
        .fullScreenModal(
            isPresented: $viewModel.isShowingMoreMenu
        ) {
            KeyListMoreMenuModal(
                isPresented: $viewModel.isShowingMoreMenu,
                isExportKeysSelected: $viewModel.isExportKeysSelected
            )
            .clearModalBackground()
        }
        .fullScreenModal(
            isPresented: $viewModel.isShowingKeysExportModal
        ) {
            ExportMultipleKeysModal(
                viewModel: .init(
                    viewModel: ExportMultipleKeysModalViewModel(
                        selectedItems: .keySets(viewModel.selectedItems),
                        count: viewModel.selectedItems.count
                    ),
                    isPresented: $viewModel.isShowingKeysExportModal
                )
            )
            .clearModalBackground()
            .onAppear {
                viewModel.selectedItems.removeAll()
                viewModel.isExportKeysSelected.toggle()
            }
        }
        .bottomSnackbar(
            viewModel.snackbarViewModel,
            isPresented: $viewModel.isSnackbarPresented
        )
    }

    func keyList() -> some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 0) {
                ForEach(
                    viewModel.listViewModel.list.sorted(by: { $0.keyName < $1.keyName }),
                    id: \.keyName
                ) {
                    keyItem($0)
                        .padding([.horizontal, .bottom], Spacing.extraSmall)
                }
                Spacer()
                    .listRowBackground(Asset.backgroundSystem.swiftUIColor)
                    .listRowSeparator(.hidden)
                    .frame(height: Heights.actionButton + Spacing.large + Heights.tabbarHeight)
            }
        }
    }

    func keyItem(_ keySetViewModel: KeySetViewModel) -> some View {
        KeySetRow(
            viewModel: keySetViewModel,
            selectedItems: $viewModel.selectedItems,
            isExportKeysSelected: $viewModel.isExportKeysSelected
        )
        .onTapGesture {
            viewModel.onKeyTap(keySetViewModel)
        }
    }

    var exportKeysOverlay: some View {
        HStack {
            Button(action: viewModel.onExportAllTap) {
                Localizable.KeySets.More.Action.exportAll.text
                    .foregroundColor(Asset.accentPink300.swiftUIColor)
                    .font(PrimaryFont.labelL.font)
            }
            .padding(.leading, Spacing.medium)
            Spacer()
            Button(action: viewModel.onExportSelectedTap) {
                Localizable.KeySets.More.Action.exportSelected.text
                    .foregroundColor(
                        viewModel.selectedItems.isEmpty ?
                            Asset.textAndIconsDisabled.swiftUIColor :
                            Asset.accentPink300
                            .swiftUIColor
                    )
                    .font(PrimaryFont.labelL.font)
            }
            .disabled(viewModel.selectedItems.isEmpty)
            .padding(.trailing, Spacing.medium)
        }
        .frame(height: Heights.tabbarHeight)
        .background(Asset.backgroundSecondary.swiftUIColor)
    }
}

extension KeySetList {
    final class ViewModel: ObservableObject {
        private let keyListService: KeyListService
        private let cancelBag = CancelBag()
        private let modelBuilder: KeySetListViewModelBuilder
        private let keyDetailsService: KeyDetailsService
        let tabBarViewModel: TabBarView.ViewModel
        var snackbarViewModel: SnackbarViewModel = .init(title: "")
        private weak var appState: AppState!
        @Published var dataModel: MSeeds = .init(seedNameCards: [])
        @Published var listViewModel: KeySetListViewModel = .init(list: [])
        @Published var selectedItems: [KeySetViewModel] = []
        @Published var detailsToPresent: MKeysNew?
        @Published var detailsKeyName: String = ""

        @Published var isShowingRecoverKeySet = false

        @Published var isShowingKeysExportModal = false
        @Published var isSnackbarPresented: Bool = false
        @Published var isShowingKeyDetails = false
        @Published var isShowingNewSeedMenu = false
        @Published var isShowingCreateKeySet = false
        @Published var isShowingMoreMenu = false
        @Published var isExportKeysSelected = false
        @Published var shouldShowCreateKeySet = false
        @Published var shouldShowRecoverKeySet = false
        @Published var isShowingDetails = false

        init(
            keyDetailsService: KeyDetailsService = KeyDetailsService(),
            keyListService: KeyListService = KeyListService(),
            modelBuilder: KeySetListViewModelBuilder = KeySetListViewModelBuilder(),
            tabBarViewModel: TabBarView.ViewModel
        ) {
            self.keyDetailsService = keyDetailsService
            self.keyListService = keyListService
            self.modelBuilder = modelBuilder
            self.tabBarViewModel = tabBarViewModel
            updateData()
        }

        func use(appState: AppState) {
            self.appState = appState
            appState.userData.$keyListRequiresUpdate.sink { [weak self] requiresUpdate in
                guard requiresUpdate else { return }
                self?.updateData()
            }.store(in: cancelBag)
        }

        func updateView(_ dataModel: MSeeds) {
            listViewModel = modelBuilder.build(for: dataModel)
        }

        func updateData() {
            dataModel = keyListService.getKeyList()
            updateView(dataModel)
        }

        func loadKeysInformation(
            for seedName: String,
            _ completion: @escaping (Result<MKeysNew, ServiceError>) -> Void
        ) {
            keyDetailsService.getKeys(for: seedName, completion)
        }

        func onKeyDetailsCompletion(_ completionAction: KeyDetailsView.OnCompletionAction) {
            switch completionAction {
            case .keySetDeleted:
                updateData()
                snackbarViewModel = .init(
                    title: Localizable.KeySetsModal.Confirmation.snackbar.string,
                    style: .warning
                )
                isSnackbarPresented = true
            }
        }

        func onKeyTap(_ viewModel: KeySetViewModel) {
            if isExportKeysSelected {
                if selectedItems.contains(viewModel) {
                    selectedItems.removeAll { $0 == viewModel }
                } else {
                    selectedItems.append(viewModel)
                }
            } else {
                loadKeysInformation(for: viewModel.keyName) { result in
                    switch result {
                    case let .success(keysData):
                        self.detailsToPresent = keysData
                        self.detailsKeyName = viewModel.keyName
                        self.isShowingDetails = true
                    case .failure:
                        self.detailsToPresent = nil
                        self.detailsKeyName = ""
                    }
                }
            }
        }

        func keyDetails() -> KeyDetailsView.ViewModel {
            .init(
                keyName: detailsKeyName,
                keysData: detailsToPresent,
                onCompletion: onKeyDetailsCompletion(_:)
            )
        }

        func onNewSeedMenuDismiss() {
            // iOS 15 handling of following .fullscreen presentation after dismissal, we need to dispatch this async
            DispatchQueue.main.async {
                if self.shouldShowCreateKeySet {
                    self.shouldShowCreateKeySet = false
                    self.isShowingCreateKeySet = true
                }
                if self.shouldShowRecoverKeySet {
                    self.shouldShowRecoverKeySet = false
                    self.isShowingRecoverKeySet = true
                }
                if self.shouldShowRecoverKeySet {
                    self.shouldShowRecoverKeySet = false
                    self.isShowingRecoverKeySet = true
                }
            }
        }

        func onExportAllTap() {
            selectedItems = listViewModel.list
            isShowingKeysExportModal = true
        }

        func onExportSelectedTap() {
            isShowingKeysExportModal = true
        }

        func onAddKeySetTap() {
            isShowingNewSeedMenu = true
        }

        func onCancelExportTap() {
            isExportKeysSelected = false
            selectedItems.removeAll()
        }

        func onShowMoreTap() {
            isShowingMoreMenu.toggle()
        }

        func onKeySetAddCompletion(_ completionAction: CreateKeysForNetworksView.OnCompletionAction) {
            let message: String
            switch completionAction {
            case let .createKeySet(seedName):
                message = Localizable.CreateKeysForNetwork.Snackbar.keySetCreated(seedName)
            case let .recoveredKeySet(seedName),
                 let .bananaSplitRecovery(seedName):
                message = Localizable.CreateKeysForNetwork.Snackbar.keySetRecovered(seedName)
            }
            snackbarViewModel = .init(
                title: message,
                style: .info
            )
            isSnackbarPresented = true
        }
    }
}

private struct KeyListEmptyState: View {
    var body: some View {
        VStack(spacing: Spacing.extraSmall) {
            Spacer()
            Text(Localizable.KeySets.Label.Empty.title.key)
                .font(PrimaryFont.titleM.font)
            Text(Localizable.KeySets.Label.Empty.subtitle.key)
                .font(PrimaryFont.bodyL.font)
            Spacer()
                .frame(height: Heights.actionButton + 2 * Spacing.large)
            Spacer()
        }
        .padding(Spacing.componentSpacer)
        .multilineTextAlignment(.center)
        .lineSpacing(Spacing.extraExtraSmall)
        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
    }
}

#if DEBUG
    struct KeySetListPreview: PreviewProvider {
        static var previews: some View {
            KeySetList(
                viewModel: .init(tabBarViewModel: .mock)
            )
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
            .environmentObject(AppState.preview)
        }
    }
#endif
