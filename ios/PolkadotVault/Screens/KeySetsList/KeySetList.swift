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
                            backgroundColor: Asset.backgroundSystem.swiftUIColor
                        )
                    )
                    switch viewModel.listViewModel {
                    case .none:
                        // Loading state, we can ignore it, as we'll transit to new design
                        Spacer()
                    case let .some(listModel):
                        // Empty state
                        if listModel.list.isEmpty {
                            KeyListEmptyState()
                        } else {
                            // List of Key Sets
                            keyList(listModel: listModel)
                        }
                    }
                }
                .background(
                    Asset.backgroundSystem.swiftUIColor
                        .ignoresSafeArea()
                )
                .navigationViewStyle(StackNavigationViewStyle())
                .navigationBarHidden(true)
                VStack {
                    VStack(spacing: 0) {
                        ConnectivityAlertOverlay(viewModel: .init())
                        PrimaryButton(
                            action: viewModel.onAddKeySetTap,
                            text: Localizable.KeySets.Action.add.key
                        )
                        .padding(.horizontal, Spacing.large)
                        .padding(.bottom, Spacing.large)
                    }
                    TabBarView(
                        viewModel: viewModel.tabBarViewModel
                    )
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
        .bottomSnackbar(
            viewModel.snackbarViewModel,
            isPresented: $viewModel.isSnackbarPresented
        )
    }

    func keyList(listModel: KeySetListViewModel) -> some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 0) {
                ForEach(
                    listModel.list.sorted(by: { $0.keyName < $1.keyName }),
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
            viewModel: keySetViewModel
        )
        .onTapGesture {
            viewModel.onKeyTap(keySetViewModel)
        }
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
        @Published var dataModel: MSeeds?
        @Published var listViewModel: KeySetListViewModel?
        @Published var detailsToPresent: MKeysNew?
        @Published var detailsKeyName: String = ""

        @Published var isShowingRecoverKeySet = false

        @Published var isSnackbarPresented: Bool = false
        @Published var isShowingKeyDetails = false
        @Published var isShowingNewSeedMenu = false
        @Published var isShowingCreateKeySet = false
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

        func updateView(_ dataModel: MSeeds?) {
            guard let dataModel = dataModel else { return }
            listViewModel = modelBuilder.build(for: dataModel)
        }

        func updateData() {
            keyListService.getKeyList { result in
                switch result {
                case let .success(seeds):
                    self.dataModel = seeds
                case .failure:
                    self.dataModel = .init(seedNameCards: [])
                }
                self.updateView(self.dataModel)
            }
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

        func onAddKeySetTap() {
            isShowingNewSeedMenu = true
        }

        func onKeySetAddCompletion(_ completionAction: CreateKeysForNetworksView.OnCompletionAction) {
            let message: String
            switch completionAction {
            case let .createKeySet(seedName):
                message = Localizable.CreateKeysForNetwork.Snackbar.keySetCreated(seedName)
            case let .recoveredKeySet(seedName):
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
