//
//  KeySetList.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 22/08/2022.
//

import SwiftUI

struct KeySetList: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject var appState: AppState
    @State private var isShowingNewSeedMenu = false
    @State private var isShowingCreateKeySet = false
    @State private var isShowingMoreMenu = false
    @State private var isExportKeysSelected = false
    @State private var shouldShowCreateKeySet = false
    @State private var shouldShowRecoverKeySet = false
    @State private var isShowingRecoverKeySet = false
    @State private var detailsToPresent: MKeysNew?
    @State private var isShowingDetails = false

    @State var selectedItems: [KeySetViewModel] = []

    var body: some View {
        NavigationView {
            ZStack(alignment: .bottom) {
                // Main screen
                VStack(spacing: 0) {
                    // Navigation Bar
                    NavigationBarView(
                        viewModel: NavigationBarViewModel(
                            title: Localizable.KeySets.title.string,
                            leftButtons: [.init(
                                type: isExportKeysSelected ? .xmark : .empty,
                                action: {
                                    isExportKeysSelected.toggle()
                                    selectedItems.removeAll()
                                }
                            )],
                            rightButtons: [.init(
                                type: isExportKeysSelected ? .empty : .more,
                                action: {
                                    isShowingMoreMenu.toggle()
                                }
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
                    if !isExportKeysSelected {
                        VStack(spacing: 0) {
                            ConnectivityAlertOverlay(viewModel: .init())
                            PrimaryButton(
                                action: {
                                    isShowingNewSeedMenu.toggle()
                                },
                                text: Localizable.KeySets.Action.add.key
                            )
                            .padding(.horizontal, Spacing.large)
                            .padding(.bottom, Spacing.large)
                        }
                    }
                    TabBarView(
                        selectedTab: .keys,
                        onQRCodeTap: viewModel.onQRCodeTap
                    )
                }
                if isExportKeysSelected {
                    exportKeysOverlay
                }
            }
        }
        .onAppear {
            viewModel.use(appState: appState)
            viewModel.updateData()
        }
        .onChange(of: isShowingDetails, perform: { _ in
            guard !isShowingDetails else { return }
            viewModel.updateData()
        })
        .fullScreenCover(
            isPresented: $isShowingNewSeedMenu,
            onDismiss: {
                // iOS 15 handling of following .fullscreen presentation after dismissal, we need to dispatch this async
                DispatchQueue.main.async {
                    if shouldShowCreateKeySet {
                        shouldShowCreateKeySet = false
                        isShowingCreateKeySet = true
                    }
                    if shouldShowRecoverKeySet {
                        shouldShowRecoverKeySet = false
                        isShowingRecoverKeySet = true
                    }
                    if shouldShowRecoverKeySet {
                        shouldShowRecoverKeySet = false
                        isShowingRecoverKeySet = true
                    }
                }
            }
        ) {
            AddKeySetModal(
                isShowingNewSeedMenu: $isShowingNewSeedMenu,
                shouldShowCreateKeySet: $shouldShowCreateKeySet,
                shouldShowRecoverKeySet: $shouldShowRecoverKeySet
            )
            .clearModalBackground()
        }
        .fullScreenCover(
            isPresented: $isShowingCreateKeySet,
            onDismiss: viewModel.updateData
        ) {
            EnterKeySetNameView(viewModel: .init(isPresented: $isShowingCreateKeySet))
        }
        .fullScreenCover(
            isPresented: $isShowingRecoverKeySet,
            onDismiss: viewModel.updateData
        ) {
            RecoverKeySetNameView(viewModel: .init(isPresented: $isShowingRecoverKeySet))
        }
        .fullScreenCover(isPresented: $isShowingMoreMenu) {
            KeyListMoreMenuModal(
                isPresented: $isShowingMoreMenu,
                isExportKeysSelected: $isExportKeysSelected
            )
            .clearModalBackground()
        }
        .fullScreenCover(
            isPresented: $viewModel.isShowingKeysExportModal
        ) {
            ExportMultipleKeysModal(
                viewModel: .init(
                    viewModel: ExportMultipleKeysModalViewModel(
                        selectedItems: .keySets(selectedItems),
                        count: selectedItems.count
                    ),
                    isPresented: $viewModel.isShowingKeysExportModal
                )
            )
            .clearModalBackground()
            .onAppear {
                selectedItems.removeAll()
                isExportKeysSelected.toggle()
            }
        }
    }

    func keyList() -> some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 0) {
                ForEach(
                    viewModel.listViewModel.list.sorted(by: { $0.keyName < $1.keyName }),
                    id: \.keyName
                ) {
                    keyItem($0)
                    NavigationLink(
                        destination:
                        KeyDetailsView(
                            viewModel: .init(
                                keyName: $0.keyName,
                                keysData: detailsToPresent
                            )
                        )
                        .navigationBarHidden(true),
                        isActive: $isShowingDetails
                    ) { EmptyView() }
                }
                Spacer()
                    .listRowBackground(Asset.backgroundSystem.swiftUIColor)
                    .listRowSeparator(.hidden)
                    .frame(height: Heights.actionButton + Spacing.large + Heights.tabbarHeight)
            }
        }
    }

    func keyItem(_ viewModel: KeySetViewModel) -> some View {
        KeySetRow(
            viewModel: viewModel,
            selectedItems: $selectedItems,
            isExportKeysSelected: $isExportKeysSelected
        )
        .padding([.horizontal, .bottom], Spacing.extraSmall)
        .onTapGesture {
            if isExportKeysSelected {
                if selectedItems.contains(viewModel) {
                    selectedItems.removeAll { $0 == viewModel }
                } else {
                    selectedItems.append(viewModel)
                }
            } else {
                self.viewModel.loadKeysInformation(for: viewModel.keyName) { result in
                    switch result {
                    case let .success(keysData):
                        detailsToPresent = keysData
                        isShowingDetails = true
                    case .failure:
                        ()
                    }
                }
            }
        }
    }

    var exportKeysOverlay: some View {
        HStack {
            Button(action: {
                selectedItems = viewModel.listViewModel.list
                viewModel.isShowingKeysExportModal.toggle()
            }) {
                Localizable.KeySets.More.Action.exportAll.text
                    .foregroundColor(Asset.accentPink300.swiftUIColor)
                    .font(PrimaryFont.labelL.font)
            }
            .padding(.leading, Spacing.medium)
            Spacer()
            Button(action: {
                viewModel.isShowingKeysExportModal.toggle()
            }) {
                Localizable.KeySets.More.Action.exportSelected.text
                    .foregroundColor(
                        selectedItems.isEmpty ?
                            Asset.textAndIconsDisabled.swiftUIColor :
                            Asset.accentPink300
                            .swiftUIColor
                    )
                    .font(PrimaryFont.labelL.font)
            }
            .disabled(selectedItems.isEmpty)
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
        private var dataModel: MSeeds
        private weak var appState: AppState!
        @Published var isShowingKeysExportModal = false
        @Published var listViewModel: KeySetListViewModel = .init(list: [])

        let onQRCodeTap: () -> Void
        let keyDetailsService: KeyDetailsService

        init(
            keyDetailsService: KeyDetailsService = KeyDetailsService(),
            keyListService: KeyListService = KeyListService(),
            modelBuilder: KeySetListViewModelBuilder = KeySetListViewModelBuilder(),
            dataModel: MSeeds,
            onQRCodeTap: @escaping () -> Void
        ) {
            self.keyDetailsService = keyDetailsService
            self.keyListService = keyListService
            self.modelBuilder = modelBuilder
            self.dataModel = dataModel
            self.onQRCodeTap = onQRCodeTap
            updateView(dataModel)
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
                viewModel: .init(dataModel: PreviewData.mseeds, onQRCodeTap: {})
            )
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
            .environmentObject(NavigationCoordinator())
            .environmentObject(AppState.preview)
        }
    }
#endif
