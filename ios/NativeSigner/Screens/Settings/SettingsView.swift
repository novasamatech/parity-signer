//
//  SettingsView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 12/12/2022.
//

import SwiftUI

struct SettingsView: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var data: SignerDataModel

    var body: some View {
        VStack(spacing: 0) {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: Localizable.Settings.Label.title.string,
                    leftButton: .empty,
                    rightButton: .empty,
                    backgroundColor: Asset.backgroundSystem.swiftUIColor
                )
            )
            ScrollView {
                VStack(spacing: 0) {
                    ForEach(viewModel.renderable.items, id: \.id) { renderable in
                        SettingsRowView(renderable: renderable)
                            .contentShape(Rectangle())
                            .onTapGesture {
                                viewModel.onTapAction(renderable.item)
                            }
                    }
                }
            }
        }
        .background(Asset.backgroundPrimary.swiftUIColor)
        .onAppear {
            viewModel.use(navigation: navigation)
            viewModel.use(data: data)
            viewModel.loadData()
        }
        .fullScreenCover(isPresented: $viewModel.isPresentingWipeConfirmation) {
            HorizontalActionsBottomModal(
                viewModel: .wipeAll,
                mainAction: viewModel.wipe(),
                isShowingBottomAlert: $viewModel.isPresentingWipeConfirmation
            )
            .clearModalBackground()
        }
    }
}

extension SettingsView {
    final class ViewModel: ObservableObject {
        @Published var renderable: SettingsViewRenderable = .init()
        @Published var isPresentingWipeConfirmation = false

        private weak var navigation: NavigationCoordinator!
        private weak var data: SignerDataModel!

        init() {}

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func use(data: SignerDataModel) {
            self.data = data
        }

        func loadData() {
            renderable = SettingsViewRenderable()
        }

        func onTapAction(_ item: SettingsItem) {
            if let action = item.detailsNavigation {
                navigation.perform(navigation: .init(action: action))
            } else {
                switch item {
                case .leaveFeedback:
                    ()
                case .appVersion:
                    ()
                case .wipe:
                    onTapWipe()
                default:
                    ()
                }
            }
        }

        private func onTapWipe() {
            isPresentingWipeConfirmation = true
        }

        func wipe() {
            data.wipe()
        }
    }
}

struct SettingsViewRenderable: Equatable {
    let items: [SettingsRowRenderable]

    init(items: [SettingsItem] = SettingsItem.allCases) {
        self.items = items
            .map { .init(item: $0, title: $0.title, isDestructive: $0.isDestructive, hasDetails: $0.hasDetails) }
    }
}

#if DEBUG
    struct SettingsView_Previews: PreviewProvider {
        static var previews: some View {
            SettingsView(viewModel: .init())
                .environmentObject(NavigationCoordinator())
                .environmentObject(SignerDataModel())
        }
    }
#endif
