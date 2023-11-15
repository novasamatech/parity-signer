//
//  ExportKeysSelectionModal.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 04/08/2023.
//

import SwiftUI

struct ExportKeysSelectionModal: View {
    private enum Constants {
        static let maxItems = 5
    }

    @StateObject var viewModel: ViewModel

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: {
                viewModel.cancelAction()
            },
            animateBackground: $viewModel.animateBackground,
            ignoredEdges: .bottom,
            content: {
                VStack(spacing: 0) {
                    // Header with X button
                    HStack {
                        Text(selectionTitle)
                            .foregroundColor(.textAndIconsPrimary)
                            .font(PrimaryFont.titleS.font)
                        Spacer()
                        CloseModalButton(action: viewModel.cancelAction)
                    }
                    .padding(.leading, Spacing.large)
                    .padding(.trailing, Spacing.medium)
                    Divider()
                        .padding(.vertical, Spacing.medium)
                    // List of items
                    if viewModel.derivedKeys.count > Constants.maxItems {
                        ScrollView {
                            itemsList()
                        }
                    } else {
                        itemsList()
                    }
                    // Bottom overlay
                    HStack {
                        // Select All
                        Button(action: { viewModel.selectAll() }) {
                            Localizable.KeyDetails.Overlay.Action.selectAll.text
                                .foregroundColor(.accentPink300)
                                .font(PrimaryFont.labelL.font)
                        }
                        .padding(.leading, Spacing.medium)
                        Spacer()
                        // Export
                        Button(action: viewModel.onExport) {
                            Localizable.KeyDetails.Overlay.Action.export.text
                                .foregroundColor(.accentPink300)
                                .font(PrimaryFont.labelL.font)
                        }
                        .padding(.trailing, Spacing.medium)
                    }
                    .frame(height: Heights.tabbarHeight)
                }
            }
        )
    }

    @ViewBuilder
    func item(for key: DerivedKeyRowModel) -> some View {
        HStack(alignment: .center, spacing: Spacing.small) {
            NetworkIdenticon(
                identicon: key.viewModel.identicon,
                network: key.viewModel.network,
                background: .backgroundPrimary,
                size: Heights.identiconInCell
            )
            .padding(.top, Spacing.extraExtraSmall)
            VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                fullPath(key.viewModel)
                    .foregroundColor(.textAndIconsTertiary)
                    .font(PrimaryFont.captionM.font)
                Text(key.viewModel.base58.truncateMiddle())
                    .foregroundColor(.textAndIconsPrimary)
                    .font(PrimaryFont.bodyL.font)
                    .lineLimit(1)
            }
            Spacer()
            VStack(alignment: .center) {
                if viewModel.isSelected(key) {
                    Image(.checkmarkChecked)
                        .foregroundColor(.accentPink300)
                } else {
                    Image(.checkmarkUnchecked)
                }
            }
            .frame(minHeight: .zero, maxHeight: .infinity)
        }
        .padding(.horizontal, Spacing.medium)
        .frame(height: Heights.exportKeysSelectionCellHeight)
        .fixedSize(horizontal: false, vertical: true)
        .onTapGesture {
            viewModel.toggleSelection(key)
        }
    }

    /// String interpolation for SFSymbols is a bit unstable if creating `String` inline by using conditional logic or
    /// `appending` from `StringProtocol`. Hence less DRY approach and dedicated function to wrap that
    private func fullPath(_ viewModel: DerivedKeyRowViewModel) -> Text {
        viewModel.hasPassword ?
            Text(
                "\(viewModel.path)\(Localizable.Shared.Label.passwordedPathDelimeter.string)\(Image(.lock))"
            ) :
            Text(viewModel.path)
    }

    @ViewBuilder
    func itemsList() -> some View {
        LazyVStack(spacing: 0) {
            VStack(alignment: .leading, spacing: 0) {
                HStack(alignment: .center, spacing: Spacing.small) {
                    if let rootIdenticon = viewModel.rootIdenticon {
                        IdenticonView(identicon: rootIdenticon)
                    }
                    Text(viewModel.rootKey.truncateMiddle())
                        .font(PrimaryFont.bodyL.font)
                        .lineLimit(1)
                    Spacer()
                    Image(.checkmarkChecked)
                }
                .foregroundColor(.textAndIconsTertiary)
                .padding(.horizontal, Spacing.medium)
            }
            .frame(height: Heights.exportKeysSelectionCellHeight)
            .fixedSize(horizontal: false, vertical: true)
            ForEach(
                viewModel.derivedKeys,
                id: \.id
            ) {
                item(for: $0)
            }
        }
    }

    var selectionTitle: String {
        let localizable = Localizable.KeyDetails.Overlay.Label.self
        let itemsCount = viewModel.selectedKeys.count + 1
        let result: String = switch itemsCount {
        case 1:
            localizable.title(String(itemsCount), localizable.Key.single.string)
        default:
            localizable.title(String(itemsCount), localizable.Key.plural.string)
        }
        return result
    }
}

extension ExportKeysSelectionModal {
    enum OnCompletionAction: Equatable {
        case onCancel
        case onKeysExport([DerivedKeyRowModel])
    }

    final class ViewModel: ObservableObject {
        private let onCompletion: (OnCompletionAction) -> Void
        @Published var animateBackground: Bool = false
        @Published var rootKey: String
        @Published var rootIdenticon: Identicon?
        @Published var derivedKeys: [DerivedKeyRowModel]
        @Published var selectedKeys: [DerivedKeyRowModel] = []
        @Binding var isPresented: Bool

        init(
            rootKey: String,
            rootIdenticon: Identicon?,
            derivedKeys: [DerivedKeyRowModel],
            isPresented: Binding<Bool>,
            onCompletion: @escaping (OnCompletionAction) -> Void
        ) {
            self.rootKey = rootKey
            self.rootIdenticon = rootIdenticon
            self.derivedKeys = derivedKeys
            _isPresented = isPresented
            self.onCompletion = onCompletion
        }

        func cancelAction() {
            Animations.chainAnimation(
                animateBackground.toggle(),
                delayedAnimationClosure: {
                    self.isPresented = false
                    self.onCompletion(.onCancel)
                }()
            )
        }

        func isSelected(_ key: DerivedKeyRowModel) -> Bool {
            selectedKeys.contains(key)
        }

        func toggleSelection(_ key: DerivedKeyRowModel) {
            if selectedKeys.contains(key) {
                selectedKeys.removeAll { $0 == key }
            } else {
                selectedKeys.append(key)
            }
        }

        func onExport() {
            Animations.chainAnimation(
                animateBackground.toggle(),
                delayedAnimationClosure: {
                    self.isPresented = false
                    self.onCompletion(.onKeysExport(self.selectedKeys))
                }()
            )
        }

        func selectAll() {
            selectedKeys = derivedKeys
        }
    }
}

#if DEBUG
    struct ExportKeysSelectionModal_Previews: PreviewProvider {
        static var previews: some View {
            ExportKeysSelectionModal(
                viewModel: .init(
                    rootKey: "",
                    rootIdenticon: .stubJdenticon,
                    derivedKeys: [],
                    isPresented: Binding<Bool>.constant(true),
                    onCompletion: { _ in }
                )
            )
        }
    }
#endif
