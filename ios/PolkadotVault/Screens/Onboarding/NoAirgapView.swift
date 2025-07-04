//
//  NoAirgapView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 15/02/2023.
//

import SwiftUI

struct NoAirgapView: View {
    @StateObject var viewModel: ViewModel

    var body: some View {
        GeometryReader { geo in
            ScrollView {
                VStack(alignment: .center, spacing: 0) {
                    // Header text
                    Text(viewModel.title)
                        .font(PrimaryFont.titleL.font)
                        .foregroundColor(.textAndIconsPrimary)
                        .multilineTextAlignment(.center)
                        .padding(.top, Spacing.extraLarge)
                        .padding(.horizontal, Spacing.large)
                    Localizable.Airgap.Label.content.text
                        .font(PrimaryFont.bodyM.font)
                        .foregroundColor(.textAndIconsTertiary)
                        .multilineTextAlignment(.center)
                        .padding(.horizontal, Spacing.medium)
                        .padding(.vertical, Spacing.medium)
                    // Airgap connectivity
                    VStack(spacing: 0) {
                        VStack(alignment: .leading, spacing: 0) {
                            cell(.aiplaneMode, isChecked: viewModel.isAirplaneModeChecked)
                                .padding(.bottom, Spacing.small)
                            Divider()
                            cell(.wifi, isChecked: viewModel.isWifiChecked)
                                .padding(.top, Spacing.small)
                                .padding(.bottom, Spacing.small)
                            Divider()
                            cell(.location, isChecked: viewModel.isLocationChecked)
                                .padding(.top, Spacing.small)
                        }
                        .padding(Spacing.medium)
                    }
                    .strokeContainerBackground()
                    .padding(.horizontal, Spacing.medium)
                    .padding(.vertical, Spacing.extraSmall)
                    // Cables connectivity
                    VStack(spacing: 0) {
                        VStack(alignment: .leading, spacing: 0) {
                            HStack(alignment: .center, spacing: Spacing.large) {
                                Image(.airgapCables)
                                    .padding(.leading, Spacing.extraSmall)
                                    .foregroundColor(.textAndIconsTertiary)
                                Localizable.Airgap.Label.cables.text
                                    .foregroundColor(.textAndIconsTertiary)
                                    .font(PrimaryFont.bodyL.font)
                            }
                            .padding(.bottom, Spacing.medium)
                            Divider()
                            HStack(alignment: .center, spacing: Spacing.large) {
                                Group {
                                    if viewModel.isCableCheckBoxSelected {
                                        Image(.checkboxChecked)
                                            .foregroundColor(.accentPink300)
                                    } else {
                                        Image(.checkboxEmpty)
                                            .foregroundColor(.textAndIconsPrimary)
                                    }
                                }
                                .padding(.leading, Spacing.extraSmall)
                                Localizable.Airgap.Label.Cables.confirmation.text
                                    .multilineTextAlignment(.leading)
                                    .fixedSize(horizontal: false, vertical: true)
                                    .foregroundColor(.textAndIconsPrimary)
                                    .font(PrimaryFont.bodyL.font)
                            }
                            .padding(.top, Spacing.medium)
                            .contentShape(Rectangle())
                            .onTapGesture {
                                viewModel.toggleCheckbox()
                            }
                        }
                        .padding(Spacing.medium)
                    }
                    .strokeContainerBackground()
                    .padding(.horizontal, Spacing.medium)
                    .padding(.vertical, Spacing.extraSmall)
                    Spacer()
                    ActionButton(
                        action: viewModel.onDoneTap,
                        text: viewModel.actionTitle,
                        style: .primary(isDisabled: $viewModel.isActionDisabled)
                    )
                    .padding(Spacing.large)
                }
                .frame(
                    minWidth: geo.size.width,
                    minHeight: geo.size.height
                )
            }
            .background(.backgroundSystem)
        }
    }

    @ViewBuilder
    func cell(_ component: AirgapComponent, isChecked: Bool) -> some View {
        HStack(alignment: .center, spacing: Spacing.medium) {
            Group {
                isChecked ? component.checkedIcon : component.uncheckedIcon
            }
            Text(component.title)
                .foregroundColor(isChecked ? component.checkedForegroundColor : component.uncheckedForegroundColor)
                .font(PrimaryFont.bodyL.font)
        }
    }
}

extension NoAirgapView {
    enum Mode {
        case onboarding
        case noAirgap
    }

    struct AirgapComponentStatus: Equatable, Hashable {
        let component: AirgapComponent
        let isChecked: Bool
    }

    final class ViewModel: ObservableObject {
        @Published var isCableCheckBoxSelected: Bool = false
        @Published var isActionDisabled: Bool = true
        @Published var isAirplaneModeChecked: Bool = false
        @Published var isWifiChecked: Bool = false
        @Published var isLocationChecked: Bool = false
        private let cancelBag = CancelBag()
        private let mode: Mode
        private let airgapMediator: AirgapMediating
        private let onActionTap: () -> Void
        var title: String {
            mode == .onboarding ? Localizable.Airgap.Label.Title.setup.string : Localizable.Airgap.Label.Title.broken
                .string
        }

        var actionTitle: LocalizedStringKey {
            mode == .onboarding ? Localizable.Airgap.Action.next.key : Localizable.Airgap.Action.done.key
        }

        init(
            mode: Mode,
            airgapMediator: AirgapMediating = ServiceLocator.airgapMediator,
            onActionTap: @escaping () -> Void
        ) {
            self.mode = mode
            self.airgapMediator = airgapMediator
            self.onActionTap = onActionTap
            subscribeToUpdates()
        }

        private func subscribeToUpdates() {
            airgapMediator.airgapPublisher
                .receive(on: DispatchQueue.main)
                .sink { [weak self] airgapState in
                    self?.isAirplaneModeChecked = airgapState.isAirplaneModeOn
                    self?.isWifiChecked = !airgapState.isWifiOn
                    self?.isLocationChecked = !airgapState.isLocationServiceEnabled
                    self?.updateActionState()
                }
                .store(in: cancelBag)
        }

        func onDoneTap() {
            onActionTap()
        }

        func toggleCheckbox() {
            isCableCheckBoxSelected.toggle()
            updateActionState()
        }

        private func updateActionState() {
            isActionDisabled = !isCableCheckBoxSelected
                || !isWifiChecked
                || !isAirplaneModeChecked
                || !isLocationChecked
        }
    }
}

#if DEBUG
    struct NoAirgapView_Previews: PreviewProvider {
        static var previews: some View {
            NoAirgapView(
                viewModel: .init(mode: .onboarding, onActionTap: {})
            )
            .preferredColorScheme(.dark)
        }
    }
#endif
